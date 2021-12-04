use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::str::FromStr;
use std::{error::Error, io::BufRead};

use anyhow::{Context, Result};

pub fn file(path: impl AsRef<Path>) -> Result<impl BufRead> {
    Ok(BufReader::new(File::open(path)?))
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
