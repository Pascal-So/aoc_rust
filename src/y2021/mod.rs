pub mod d01;
pub mod d02;
pub mod d03;
pub mod d04;

#[cfg(test)]
mod tests {
    use crate::test_task;

    test_task! { y2021, d01, example, (7, 5) }
    test_task! { y2021, d01, full, (1301, 1346) }

    test_task! { y2021, d02, example, (150, 900) }
    test_task! { y2021, d02, full, (2102357, 2101031224) }

    test_task! { y2021, d03, example, (198, 230) }
    test_task! { y2021, d03, full, (3687446, 4406844) }

    test_task! { y2021, d04, example, (4512, 1924) }
    test_task! { y2021, d04, full, (38594, 21184) }
}
