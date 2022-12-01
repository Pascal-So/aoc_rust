pub mod d01;

#[cfg(test)]
mod tests {
    use crate::test_task;

    test_task! { y2022, d01, example, (24000, 45000) }
    test_task! { y2022, d01, full, (72718, 213089) }
}
