pub mod d01;
pub mod d02;

#[cfg(test)]
mod tests {
    use crate::test_task;

    test_task! { y2019, d01, full, (3305115, 4954799) }

    test_task! { y2019, d02, full, (3562624, 8298) }
}
