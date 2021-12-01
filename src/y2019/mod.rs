pub mod d01;
pub mod d02;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::file;
    use anyhow::Result;

    #[test]
    fn test_day_01() -> Result<()> {
        let sol = d01::solve(file("data/2019/01.txt")?)?;

        assert_eq!(sol.simple_fuel, 3305115);
        assert_eq!(sol.iterated_fuel, 4954799);

        Ok(())
    }

    #[test]
    fn test_day_02() -> Result<()> {
        assert_eq!(d02::solve(file("data/2019/02.txt")?)?, (3562624, 8298));
        Ok(())
    }
}
