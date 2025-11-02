use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::str::FromStr;
use std::{error::Error, io::BufRead};

use anyhow::{anyhow, Context, Result};

pub fn file(path: impl AsRef<Path>) -> Result<impl BufRead> {
    Ok(BufReader::new(File::open(path)?))
}

pub fn file_str(path: impl AsRef<Path>) -> Result<String> {
    Ok(std::fs::read_to_string(path)?)
}

pub fn parse_entries<T>(input: &str, split_on: char) -> Result<Vec<T>>
where
    T: FromStr,
    Result<T, <T as FromStr>::Err>: anyhow::Context<T, <T as FromStr>::Err>,
{
    input
        .split(split_on)
        .enumerate()
        .filter(|(_, entry)| !entry.is_empty())
        .map(|(idx, entry)| {
            entry
                .trim()
                .parse::<T>()
                .context(format!("parsing entry {}", idx + 1))
        })
        .collect()
}

pub fn split_entries(input: &str, split_on: char) -> impl Iterator<Item = &str> {
    input.split(split_on).filter(|entry| !entry.is_empty())
}

pub fn parse_iter<B, T>(buf: B, sep: u8, skip_empty: bool) -> impl Iterator<Item = Result<T>>
where
    B: BufRead,
    T: FromStr,
    <T as FromStr>::Err: Error + Send + Sync + 'static,
{
    buf.split(sep).zip(1..).filter_map(move |(entry, nr)| {
        (|| {
            let s = String::from_utf8(entry.context(format!("Cannot read entry {}", nr))?)
                .context(format!("Cannot read entry {}", nr))?;
            let trimmed = s.trim();
            if trimmed.is_empty() && skip_empty {
                Ok(None)
            } else {
                Ok(Some(trimmed.parse::<T>()?))
            }
        })()
        .transpose()
    })
}

pub fn parse_vec<B, T>(buf: B, sep: u8, skip_empty: bool) -> Result<Vec<T>>
where
    B: BufRead,
    T: FromStr,
    <T as FromStr>::Err: Error + Send + Sync + 'static,
{
    parse_iter(buf, sep, skip_empty).collect()
}

pub fn get_first_line_string<B>(buf: B) -> Result<String>
where
    B: BufRead,
{
    Ok(buf
        .lines()
        .next()
        .ok_or_else(|| anyhow!("Empty input!"))??)
}

pub fn get_first_line_bytes<B>(buf: B) -> Result<Vec<u8>>
where
    B: BufRead,
{
    Ok(buf
        .split(b'\n')
        .next()
        .ok_or_else(|| anyhow!("Empty input!"))??)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_vec() -> Result<()> {
        let invalid_utf8: &[u8] = &[0xc3, 0x28];
        assert!(parse_vec::<_, i32>(invalid_utf8, b'\n', false).is_err());

        assert_eq!(
            parse_vec::<_, i32>("12\n 1 \n4 \n".as_bytes(), b'\n', false)?,
            vec![12, 1, 4]
        );

        Ok(())
    }
}
