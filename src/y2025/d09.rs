use std::{collections::BTreeMap, ops::Range};

use anyhow::Result;

pub fn solve(input: &str) -> Result<(i64, i64)> {
    let red_tiles = parse(input);

    // Solve part 1    O(N²)
    let mut largest_area = 1;
    for (i, (x, y)) in red_tiles.iter().enumerate() {
        for (ox, oy) in &red_tiles[..i] {
            let lx = (ox - x).abs() + 1;
            let ly = (oy - y).abs() + 1;
            let area = lx * ly;
            largest_area = largest_area.max(area);
        }
    }

    // Solve part 2     O(N²log²N)
    //
    // The idea is to use a 2d Fenwick tree to query the filled-in area
    // for all rectangles, and only consider the rectangles where the
    // query returns the full expected area.

    let comp_x = coordinate_compression(red_tiles.iter().map(|(x, _)| *x));
    let comp_y = coordinate_compression(red_tiles.iter().map(|(_, y)| *y));

    // We know that the winding direction is positive (right-hand-rule) from
    // a quick manual check of the data, for both the example and the full
    // input. Can't be bothered to automate that..
    //
    // Note that we also assume that the loop does not self-intersect, which
    // is not explicitly stated in the task description but it seems like
    // we can assume that?
    //
    // Additionally we use the fact that the cardinal axis always strictly
    // alternates. This can be tested with:
    // awk -F"," 'NR % 2 == 0 { print $1; }' d09_full.txt | uniq -c | sort -n
    //
    // The annoying part is that lines have a thickness, so we don't have
    // an easy way of mapping the actual area of red+green tiles. Instead
    // we simply imagine a middle line going between centres of red tiles,
    // and map the area inside the polygon formed by that line.
    let compressed_x_size = comp_x.last_key_value().unwrap().1 + 1;
    let compressed_y_size = comp_y.last_key_value().unwrap().1 + 1;
    let mut ft = Fenwick2d::new(compressed_x_size as usize, compressed_y_size as usize);

    for i in 0..red_tiles.len() {
        let j = (i + 1) % red_tiles.len();

        let (xa, ya) = red_tiles[i];
        let (xb, yb) = red_tiles[j];

        let is_horizontal_line = ya == yb;
        if !is_horizontal_line {
            continue;
        }

        let xa = *comp_x.get(&xa).unwrap();
        let xb = *comp_x.get(&xb).unwrap();
        let y = *comp_y.get(&ya).unwrap();

        // Interestingly enough these operations are the same for
        // both lower and upper edges, since the signs swap at the
        // same time as the x coordinates swap :)
        ft.insert_point(xa, y, 1);
        ft.insert_point(xb, y, -1);
    }

    let mut largest_redgreen_area = 1;
    for (i, (x, y)) in red_tiles.iter().cloned().enumerate() {
        for (ox, oy) in red_tiles[..i].iter().cloned() {
            let lx = (ox - x).abs() + 1;
            let ly = (oy - y).abs() + 1;

            let area = lx * ly;

            if area > largest_redgreen_area {
                let x = *comp_x.get(&x).unwrap();
                let y = *comp_y.get(&y).unwrap();
                let ox = *comp_x.get(&ox).unwrap();
                let oy = *comp_y.get(&oy).unwrap();
                let range_x = x.min(ox)..x.max(ox);
                let range_y = y.min(oy)..y.max(oy);
                let expected_filled_area = (x - ox).abs() * (y - oy).abs();
                let filled_area = ft.query_rectangle(range_x, range_y);
                if filled_area == expected_filled_area {
                    largest_redgreen_area = area;
                }
            }
        }
    }

    Ok((largest_area, largest_redgreen_area))
}

/// Compress coordinates by moving everything together
fn coordinate_compression(coords: impl Iterator<Item = i64>) -> BTreeMap<i64, i64> {
    let mut map = BTreeMap::from_iter(coords.map(|c| (c, 0)));

    for (compressed_coord, val) in map.values_mut().enumerate() {
        *val = compressed_coord as i64;
    }

    map
}

/// Two-dimensional Fenwick tree
struct Fenwick2d {
    /// data for prefix sum of raw terms
    data: Vec<Vec<i64>>,
    /// data for prefix sum of terms * x
    data_x: Vec<Vec<i64>>,
    /// data for prefix sum of terms * y
    data_y: Vec<Vec<i64>>,
    /// data for prefix sum of terms * x * y
    data_xy: Vec<Vec<i64>>,
}

impl Fenwick2d {
    pub fn new(size_x: usize, size_y: usize) -> Self {
        let zeros: Vec<Vec<i64>> = (0..size_x).map(|_| vec![0; size_y]).collect();
        Self {
            data: zeros.clone(),
            data_x: zeros.clone(),
            data_y: zeros.clone(),
            data_xy: zeros,
        }
    }

    /// Query the sum across all prefix sums for the
    /// rectangle 0,0 to x,y inclusive
    pub fn query_point(&self, x: i64, y: i64) -> i64 {
        let mut out = 0;

        for xi in fenwick_queries(x) {
            for yi in fenwick_queries(y) {
                out += (x + 1) * (y + 1) * self.data[xi][yi];
                out -= (y + 1) * self.data_x[xi][yi];
                out -= (x + 1) * self.data_y[xi][yi];
                out += self.data_xy[xi][yi];
            }
        }

        out
    }

    pub fn query_rectangle(&self, x: Range<i64>, y: Range<i64>) -> i64 {
        let sx = x.start;
        let sy = y.start;
        let ex = x.end - 1;
        let ey = y.end - 1;
        let mut out = self.query_point(ex, ey);
        if sx > 0 {
            out -= self.query_point(sx - 1, ey);
        }
        if sy > 0 {
            out -= self.query_point(ex, sy - 1);
        }
        if sx > 0 && sy > 0 {
            out += self.query_point(sx - 1, sy - 1);
        }

        out
    }

    /// Add a value to a point in the prefix sum
    pub fn insert_point(&mut self, x: i64, y: i64, diff: i64) {
        for xi in fenwick_updates(x, self.data.len()) {
            for yi in fenwick_updates(y, self.data[xi].len()) {
                let x = x as i64;
                let y = y as i64;

                self.data[xi][yi] += diff;
                self.data_x[xi][yi] += x * diff;
                self.data_y[xi][yi] += y * diff;
                self.data_xy[xi][yi] += x * y * diff;
            }
        }
    }

    /// Query only the raw prefix sum, not the sum over an area
    /// of prefix sum values.
    #[cfg(test)]
    fn query_prefix_sum_point(&self, x: i64, y: i64) -> i64 {
        let mut out = 0;
        for xi in fenwick_queries(x) {
            for yi in fenwick_queries(y) {
                out += self.data[xi][yi];
            }
        }
        out
    }
}

// Inside the iterator functions we use the 1-based coordinate
// system, but everywhere else is zero-based.

/// Points where we have to access the Fenwick
/// datastructure when querying.
fn fenwick_queries(x: i64) -> impl IntoIterator<Item = usize> {
    let mut q = x + 1;
    std::iter::from_fn(move || {
        if q > 0 {
            let res = Some((q - 1) as usize);
            q -= q & -q;
            res
        } else {
            None
        }
    })
}
/// Points where we have to access the Fenwick
/// datastructure when updating.
fn fenwick_updates(x: i64, len: usize) -> impl IntoIterator<Item = usize> {
    let mut q = x + 1;
    let len = (len + 1) as i64;
    std::iter::from_fn(move || {
        if q < len {
            let res = Some((q - 1) as usize);
            q += q & -q;
            res
        } else {
            None
        }
    })
}

fn parse(input: &str) -> Vec<(i64, i64)> {
    input
        .lines()
        .map(|line| {
            let mut iter = line.split(",");
            let x = iter.next().unwrap().parse().unwrap();
            let y = iter.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fenwick_simple_square() {
        let mut ft = Fenwick2d::new(20, 10);

        assert_eq!(ft.query_point(19, 9), 0, "initial value should be zero");

        // add a 3x2 square
        ft.insert_point(12, 4, 1);
        ft.insert_point(15, 4, -1);
        ft.insert_point(12, 6, -1);
        ft.insert_point(15, 6, 1);

        // simple prefix sum should serve as inside/outside detection
        for x in 0..20 {
            for y in 0..10 {
                let inside_square = (12..15).contains(&x) && (4..6).contains(&y);
                let expected = if inside_square { 1 } else { 0 };

                assert_eq!(ft.query_prefix_sum_point(x, y), expected);
            }
        }

        // Sum over prefix sums should yield area of
        // rectangle intersected with query.
        assert_eq!(ft.query_point(19, 9), 6);
        assert_eq!(ft.query_point(19, 6), 6);
        assert_eq!(ft.query_point(19, 5), 6);
        assert_eq!(ft.query_point(19, 4), 3);
        assert_eq!(ft.query_point(19, 3), 0);

        assert_eq!(ft.query_rectangle(3..19, 0..8), 6);
    }
}
