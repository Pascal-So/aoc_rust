use std::io::BufRead;

use anyhow::{anyhow, bail, Context, Result};
use bitvec::{order::Msb0, prelude::BitVec, slice::BitSlice};
use deku::prelude::*;

type LiteralContainer = BitVec<Msb0, u64>;

fn decode_literal(lit: &LiteralContainer) -> u64 {
    let contents = lit.as_raw_slice()[0];
    contents >> (64 - lit.len())
}

#[derive(PartialEq, Debug, DekuRead)]
struct Packet {
    #[deku(bits = 3)]
    pub version: u8,

    #[deku(bits = 3)]
    pub packet_type: u8,

    #[deku(ctx = "*packet_type")]
    pub body: PacketBody,
}

impl Packet {
    fn version_sum(&self) -> i32 {
        let mut sum = self.version as i32;
        if let PacketBody::Operator(op) = &self.body {
            sum += op.packets().iter().map(Packet::version_sum).sum::<i32>();
        }
        sum
    }

    fn eval(&self) -> Result<u64> {
        let res = match &self.body {
            PacketBody::Literal(lit) => decode_literal(lit),
            PacketBody::Operator(sub) => {
                let mut evaluated = sub
                    .packets()
                    .iter()
                    .map(Packet::eval)
                    .collect::<Result<Vec<_>>>()?
                    .into_iter();

                match &self.packet_type {
                    0 => evaluated.sum::<u64>(),
                    1 => evaluated.product::<u64>(),
                    2 => evaluated.min().unwrap_or(0),
                    3 => evaluated.max().unwrap_or(0),
                    5..=7 => (|| -> Option<u64> {
                        let a = evaluated.next()?;
                        let b = evaluated.next()?;
                        let res = match &self.packet_type {
                            5 => a > b,
                            6 => a < b,
                            7 => a == b,
                            _ => unreachable!(),
                        };
                        Some(res as u64)
                    })()
                    .unwrap_or(0),
                    _ => bail!("unexpected packet type: {}", self.packet_type),
                }
            }
        };

        Ok(res)
    }
}

#[derive(PartialEq, Debug, DekuRead)]
#[deku(ctx = "packet_type: u8", id = "packet_type")]
enum PacketBody {
    #[deku(id = "0b100")]
    Literal(#[deku(reader = "literal_parser(deku::rest)")] LiteralContainer),

    #[deku(id_pat = "_")]
    Operator(SubPackets),
}

#[derive(PartialEq, Debug, DekuRead)]
#[deku(type = "bool", bits = "1")]
enum SubPackets {
    #[deku(id = "false")]
    LengthDelimited {
        #[deku(bits = 15, endian = "big")]
        length: u16,

        #[deku(bits_read = "length")]
        packets: Vec<Packet>,
    },

    #[deku(id = "true")]
    CountDelimited {
        #[deku(bits = 11, endian = "big")]
        count: u16,

        #[deku(count = "count")]
        packets: Vec<Packet>,
    },
}

impl SubPackets {
    fn packets(&self) -> &[Packet] {
        match &self {
            SubPackets::LengthDelimited { length: _, packets } => packets.as_ref(),
            SubPackets::CountDelimited { count: _, packets } => packets.as_ref(),
        }
    }
}

fn literal_parser(
    rest: &BitSlice<Msb0, u8>,
) -> Result<(&BitSlice<Msb0, u8>, LiteralContainer), DekuError> {
    let mut out = BitVec::new();
    for i in 0.. {
        let chunk = &rest[i * 5..(i + 1) * 5];
        out.extend(&chunk[1..]);

        if !chunk[0] {
            break;
        }
    }

    let consumed_bits = 5 * out.len() / 4;
    Ok((&rest[consumed_bits..], out))
}

fn decode_hex(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}

pub fn solve(buf: impl BufRead) -> Result<(i32, u64)> {
    let line = buf
        .split(b'\n')
        .next()
        .and_then(Result::ok)
        .ok_or_else(|| anyhow!("Cannot read first line of input!"))?;

    let mut input: Vec<u8> = Vec::with_capacity((line.len() + 1) / 2);
    for (i, hex) in line.into_iter().enumerate() {
        let val = decode_hex(hex).ok_or_else(|| anyhow!("Invalid hex value: '{}'", hex))?;
        if i % 2 == 0 {
            input.push(val << 4);
        } else {
            input[i / 2] += val;
        }
    }

    let packet = Packet::from_bytes((input.as_ref(), 0))
        .context("Cannot decode packet")?
        .1;

    Ok((packet.version_sum(), packet.eval()?))
}
