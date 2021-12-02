pub mod d01;
pub mod d02;

#[cfg(test)]
mod tests {
    use crate::test_task;

    test_task! { super::d01, y2021, d01, example, (7, 5) }
    test_task! { super::d01, y2021, d01, full, (1301, 1346) }

    test_task! { super::d02, y2021, d02, example, (150, 900) }
    test_task! { super::d02, y2021, d02, full, (2102357, 2101031224) }
}
