pub mod d01;

#[cfg(test)]
mod tests {
    use crate::test_task;

    test_task! { y2025, d01, example, (3, 6) }
    test_task! { y2025, d01, full, (1071, 6700) }
}
