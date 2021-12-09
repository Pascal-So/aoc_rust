pub mod d01;
pub mod d02;
pub mod d03;
pub mod d25;

#[cfg(test)]
mod tests {
    use crate::test_task;

    test_task! { y2020, d01, example, (514579, 241861950) }
    test_task! { y2020, d01, full, (921504, 195700142) }

    test_task! { y2020, d02, example, (2, 1) }
    test_task! { y2020, d02, full, (458, 342) }

    test_task! { y2020, d03, example, (7, 336) }
    test_task! { y2020, d03, full, (274, 6050183040) }

    test_task! { y2020, d25, example, 14897079 }
    test_task! { y2020, d25, full, 5414549 }
}
