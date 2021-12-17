pub mod d01;
pub mod d02;
pub mod d03;
pub mod d04;
pub mod d05;
pub mod d06;
pub mod d07;
pub mod d08;
pub mod d09;
pub mod d10;
pub mod d11;
pub mod d12;

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

    test_task! { y2021, d05, example, (5, 12) }
    test_task! { y2021, d05, full, (6189, 19164) }

    test_task! { y2021, d06, example, (5934, 26984457539) }
    test_task! { y2021, d06, full, (386536, 1732821262171) }

    test_task! { y2021, d08, example, (26, 61229) }
    test_task! { y2021, d08, full, (409, 1024649) }

    test_task! { y2021, d09, example, (15, 1134) }
    test_task! { y2021, d09, full, (600, 987840) }

    test_task! { y2021, d10, example, (26397, 288957) }
    test_task! { y2021, d10, full, (364389, 2870201088) }

    test_task! { y2021, d11, example, (1656, 195) }
    test_task! { y2021, d11, full, (1747, 505) }

    test_task! { y2021, d12, example_1, (10, 36) }
    test_task! { y2021, d12, example_2, (19, 103) }
    test_task! { y2021, d12, example_3, (226, 3509) }
    test_task! { y2021, d12, full, (4338, 114189) }
}
