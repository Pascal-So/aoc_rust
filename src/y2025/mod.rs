pub mod d01;
pub mod d02;
pub mod d03;
pub mod d04;
pub mod d05;
pub mod d06;
pub mod d07;
pub mod d08;
pub mod d09;

#[cfg(test)]
mod tests {
    use crate::test_task;

    test_task! { y2025, d01, example, (3, 6) }
    test_task! { y2025, d01, full, (1071, 6700) }

    test_task! { y2025, d02, example, (1227775554, 4174379265) }
    test_task! { y2025, d02, full, (41294979841, 66500947346) }

    test_task! { y2025, d03, example, (357, 3121910778619) }
    test_task! { y2025, d03, full, (17408, 172740584266849) }

    test_task! { y2025, d04, example, (13, 43) }
    test_task! { y2025, d04, full, (1602, 9518) }

    test_task! { y2025, d05, example, (3, 14) }
    test_task! { y2025, d05, full, (798, 366181852921027) }

    test_task! { y2025, d06, example, (4277556, 3263827) }
    test_task! { y2025, d06, full, (6725216329103, 10600728112865) }

    test_task! { y2025, d07, example, (21, 40) }
    test_task! { y2025, d07, full, (1656, 76624086587804) }

    test_task! { y2025, d08, example, (40, 25272) }
    test_task! { y2025, d08, full, (75582, 59039696) }

    test_task! { y2025, d09, example, (50, 24) }
    test_task! { y2025, d09, full, (4752484112, 1465767840) }
}
