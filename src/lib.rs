#![warn(clippy::all)]

pub mod bivariate_polynomial;
pub mod diophantine;
pub mod io;
pub mod parse;
pub mod union_find;

pub mod y2019;
pub mod y2020;
pub mod y2021;
pub mod y2022;
pub mod y2023;

#[macro_export]
macro_rules! test_task {
    ($year:ident, $day:ident, $suffix:ident, ($sol_a:expr, _)) => {
        paste::paste! {
            #[test]
            fn [<test_ $day _ $suffix>]() -> anyhow::Result<()> {
                let path = format!("data/{}/{}_{}.txt", stringify!($year), stringify!($day), stringify!($suffix));
                let sol = crate::$year::$day::solve(&crate::io::file_str(path)?)?;
                assert_eq!(sol.0, $sol_a);
                Ok(())
            }
        }
    };
    ($year:ident, $day:ident, $suffix:ident, (_, $sol_b:expr)) => {
        paste::paste! {
            #[test]
            fn [<test_ $day _ $suffix>]() -> anyhow::Result<()> {
                let path = format!("data/{}/{}_{}.txt", stringify!($year), stringify!($day), stringify!($suffix));
                let sol = crate::$year::$day::solve(&crate::io::file_str(path)?)?;
                assert_eq!(sol.1, $sol_b);
                Ok(())
            }
        }
    };
    ($year:ident, $day:ident, $suffix:ident, $solution:expr) => {
        paste::paste! {
            #[test]
            fn [<test_ $day _ $suffix>]() -> anyhow::Result<()> {
                let path = format!("data/{}/{}_{}.txt", stringify!($year), stringify!($day), stringify!($suffix));
                let sol = crate::$year::$day::solve(&crate::io::file_str(path)?)?;
                assert_eq!(sol, $solution);
                Ok(())
            }
        }
    };
}
