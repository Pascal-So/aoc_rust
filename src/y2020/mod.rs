pub mod d01;
pub mod d02;

#[cfg(test)]
mod tests {
    use crate::test_task;

    test_task! { y2020, d01, example, (514579, 241861950) }
    test_task! { y2020, d01, full, (921504, 195700142) }

    test_task! { y2020, d02, example, (2, 1) }
    test_task! { y2020, d02, full, (458, 342) }
}
