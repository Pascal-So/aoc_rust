pub mod d01;
pub mod d02;
pub mod d04;

#[cfg(test)]
mod tests {
    use crate::test_task;

    test_task! { y2019, d01, full, (3305115, 4954799) }

    test_task! { y2019, d02, full, (3562624, 8298) }

    test_task! { y2019, d04, full, (1873, 1264) }
}
