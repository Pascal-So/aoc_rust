use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

pub fn parse_vector<T: BufRead, S: FromStr>(buf: T, sep: u8) -> Result<Vec<S>, String> {
    buf.split(sep)
        .enumerate()
        .filter_map(|(i, entry)| {
            let entry_nr = i + 1;

            let entry = match entry.map(String::from_utf8) {
                Err(e) => return Err(format!("Cannot read entry {}, {}.", entry_nr, e)).into(),
                Ok(Err(e)) => return Err(format!("Cannot read entry {}, {}.", entry_nr, e)).into(),
                Ok(Ok(v)) => v,
            };

            let trimmed = entry.trim();

            if trimmed.is_empty() {
                None
            } else {
                Some(
                    trimmed
                        .parse::<S>()
                        .map_err(|_| format!("Cannot parse entry {}: '{}'", entry_nr, entry)),
                )
            }
        })
        .collect()
}

pub fn parse_vector_from_stdin<S: FromStr>(sep: u8) -> Result<Vec<S>, String> {
    parse_vector(io::stdin().lock(), sep)
}

pub fn parse_vector_from_file<S: FromStr>(
    file: impl AsRef<std::path::Path>,
    sep: u8,
) -> Result<Vec<S>, String> {
    let file = File::open(file).map_err(|e| e.to_string())?;
    let buf_reader = io::BufReader::new(file);

    parse_vector(buf_reader, sep)
}

pub fn parse_lines<T: BufRead, S: FromStr>(buf: T) -> Result<Vec<S>, String> {
    parse_vector(buf, b'\n')
}

pub fn parse_lines_from_stdin<S: FromStr>() -> Result<Vec<S>, String> {
    parse_vector_from_stdin(b'\n')
}

pub fn parse_lines_from_file<S: FromStr>(
    file: impl AsRef<std::path::Path>,
) -> Result<Vec<S>, String> {
    parse_vector_from_file(file, b'\n')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_lines() {
        let invalid_utf8: &[u8] = &[0xc3, 0x28];
        assert_eq!(
            parse_lines::<_, i32>(invalid_utf8),
            Err("Cannot read entry 1, invalid utf-8 sequence of 1 bytes from index 0.".to_owned())
        );

        assert_eq!(
            parse_lines::<_, i32>("12\n\n  1 \n   \n4".as_bytes()),
            Ok(vec![12, 1, 4])
        );
    }

    #[test]
    fn test_parse_vector() {
        assert_eq!(
            parse_vector::<_, i32>("12,,  1 ,   ,4".as_bytes(), b','),
            Ok(vec![12, 1, 4])
        );
    }
}
