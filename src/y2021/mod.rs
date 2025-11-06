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
pub mod d13;
pub mod d14;
pub mod d15;
pub mod d16;
pub mod d17;
// pub mod d18;
pub mod d20;
pub mod d21;
pub mod d22;
pub mod d23;
pub mod d24;
pub mod d25;

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

    //     test_task! { y2021, d06, example, (5934, 26984457539) }
    //     test_task! { y2021, d06, full, (386536, 1732821262171) }

    //     test_task! { y2021, d08, example, (26, 61229) }
    //     test_task! { y2021, d08, full, (409, 1024649) }

    //     test_task! { y2021, d09, example, (15, 1134) }
    //     test_task! { y2021, d09, full, (600, 987840) }

    //     test_task! { y2021, d10, example, (26397, 288957) }
    //     test_task! { y2021, d10, full, (364389, 2870201088) }

    //     test_task! { y2021, d11, example, (1656, 195) }
    //     test_task! { y2021, d11, full, (1747, 505) }

    //     test_task! { y2021, d12, example_1, (10, 36) }
    //     test_task! { y2021, d12, example_2, (19, 103) }
    //     test_task! { y2021, d12, example_3, (226, 3509) }
    //     test_task! { y2021, d12, full, (4338, 114189) }

    //     test_task! { y2021, d13, example, 17 }
    //     test_task! { y2021, d13, full, 818 }

    //     test_task! { y2021, d14, example, (1588, 2188189693529) }
    //     test_task! { y2021, d14, full, (3342, 3776553567525) }

    //     test_task! { y2021, d15, example, (40, 315) }
    //     test_task! { y2021, d15, full, (447, 2825) }

    //     test_task! { y2021, d16, example_a1, (16, _) }
    //     test_task! { y2021, d16, example_a2, (12, _) }
    //     test_task! { y2021, d16, example_a3, (23, _) }
    //     test_task! { y2021, d16, example_a4, (31, _) }
    //     test_task! { y2021, d16, example_b1, (_, 3) }
    //     test_task! { y2021, d16, example_b2, (_, 54) }
    //     test_task! { y2021, d16, example_b3, (_, 7) }
    //     test_task! { y2021, d16, example_b4, (_, 9) }
    //     test_task! { y2021, d16, example_b5, (_, 1) }
    //     test_task! { y2021, d16, example_b6, (_, 0) }
    //     test_task! { y2021, d16, example_b7, (_, 0) }
    //     test_task! { y2021, d16, example_b8, (_, 1) }
    //     test_task! { y2021, d16, full, (955, 158135423448) }

    //     test_task! { y2021, d17, example, (45, 112) }
    //     test_task! { y2021, d17, full, (2775, 1566) }

    //     test_task! { y2021, d18, example, (4140, 3993) }
    //     test_task! { y2021, d18, full, (4347, 4721) }

    //     test_task! { y2021, d20, example, (35, 3351) }
    //     test_task! { y2021, d20, full, (4873, 16394) }

    test_task! { y2021, d21, example, (739785, 444356092776315) }
    test_task! { y2021, d21, full, (1006866, 273042027784929) }

    test_task! { y2021, d22, example_1, (39, _) }
    test_task! { y2021, d22, example_2, (590784, _) }
    test_task! { y2021, d22, example_3, (474140, 2758514936282235) }
    test_task! { y2021, d22, full, (537042, 1304385553084863) }

    //     test_task! { y2021, d23, example, (12521, 44169) }
    //     test_task! { y2021, d23, full, (14348, 40954) }

    test_task! { y2021, d24, full, (45989929946199, 11912814611156) }

    test_task! { y2021, d25, example, (58, _) }
    test_task! { y2021, d25, full, (565, _) }
}
