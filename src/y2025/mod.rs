pub mod d01;
pub mod d02;

#[cfg(test)]
mod tests {
    use crate::test_task;

    test_task! { y2025, d01, example, (3, 6) }
    test_task! { y2025, d01, full, (1071, 6700) }

    test_task! { y2025, d02, example, (1227775554, 4174379265) }
    test_task! { y2025, d02, full, (41294979841, 66500947346) }
}
