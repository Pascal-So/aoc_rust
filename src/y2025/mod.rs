pub mod d01;
pub mod d02;
pub mod d03;
pub mod d04;

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
}
