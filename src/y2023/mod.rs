pub mod d01;

#[cfg(test)]
mod tests {
    use crate::test_task;

    test_task! { y2023, d01, example_a, (142, _) }
    test_task! { y2023, d01, example_b, (_, 281) }
    test_task! { y2023, d01, full, (55607, 55291) }
}
