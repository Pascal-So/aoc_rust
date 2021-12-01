pub mod d01;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::file;
    use anyhow::Result;

    #[test]
    fn test_day_01() -> Result<()> {
        let example_sol =
            d01::solve("199\n200\n208\n210\n200\n207\n240\n269\n260\n263".as_bytes())?;
        assert_eq!(
            example_sol,
            d01::Solution {
                raw: 7,
                windowed: 5
            }
        );

        let sol = d01::solve(file("data/2021/01.txt")?)?;
        dbg!(sol);
        Ok(())
    }
}
