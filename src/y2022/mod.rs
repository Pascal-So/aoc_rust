pub mod d01;
pub mod d02;

#[cfg(test)]
mod tests {
    use crate::test_task;

    test_task! { y2022, d01, example, (24000, 45000) }
    test_task! { y2022, d01, full, (72718, 213089) }

    test_task! { y2022, d02, example, (15, 12) }
    test_task! { y2022, d02, full, (8933, 11998) }
}
