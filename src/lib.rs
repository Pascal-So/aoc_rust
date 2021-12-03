#[macro_use]
extern crate impl_ops;

pub mod bivariate_polynomial;
pub mod diophantine;
pub mod io;

pub mod y2019;
pub mod y2021;

#[macro_export]
macro_rules! test_task {
    ($year:ident, $day:ident, $suffix:ident, $solution:expr) => {
        paste::paste! {
            #[test]
            fn [<test_ $day _ $suffix>]() -> anyhow::Result<()> {
                let path = format!("data/{}/{}_{}.txt", stringify!($year), stringify!($day), stringify!($suffix));
                let sol = crate::$year::$day::solve(crate::io::file(path)?)?;
                assert_eq!(sol, $solution);
                Ok(())
            }
        }
    };
}
