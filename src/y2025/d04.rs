use anyhow::Result;

pub fn solve(input: &str) -> Result<(i32, i32)> {
    let mut map = parse(input);
    let mut neighbours = count_neighbours(&map);

    let mut total_removals;
    let mut new_removals;

    (new_removals, neighbours) = removal_round(neighbours, &mut map);
    total_removals = new_removals;
    let a = total_removals;

    while new_removals > 0 {
        (new_removals, neighbours) = removal_round(neighbours, &mut map);

        total_removals += new_removals;
    }

    Ok((a, total_removals))
}

/// Count the number of active neighbours for every field
fn count_neighbours(map: &[Vec<bool>]) -> Vec<Vec<i32>> {
    let mut neighbours: Vec<Vec<i32>> = map.iter().map(|line| vec![0; line.len()]).collect();

    for (i, line) in map.iter().enumerate() {
        for (j, _) in line.iter().enumerate() {
            for di in -1..=1 {
                for dj in -1..=1 {
                    if (di, dj) == (0, 0) {
                        continue;
                    }

                    let ni = i as i32 + di;
                    let nj = j as i32 + dj;

                    if ni < 0 || nj < 0 || ni >= map.len() as i32 || nj >= line.len() as i32 {
                        continue;
                    }

                    if map[ni as usize][nj as usize] {
                        neighbours[i][j] += 1;
                    }
                }
            }
        }
    }

    neighbours
}

/// Do one round of removals, modifying the map in place.
///
/// Returns the number of removals and the new list of neighbour counts
fn removal_round(neighbours: Vec<Vec<i32>>, map: &mut [Vec<bool>]) -> (i32, Vec<Vec<i32>>) {
    let mut new_neighbours = neighbours.clone();
    let mut nr_removals = 0;

    let maplen = map.len();

    for (i, line) in map.iter_mut().enumerate() {
        let linelen = line.len();
        for (j, b) in line.iter_mut().enumerate() {
            if *b && neighbours[i][j] < 4 {
                nr_removals += 1;
                *b = false;

                for di in -1..=1 {
                    for dj in -1..=1 {
                        if (di, dj) == (0, 0) {
                            continue;
                        }

                        let ni = i as i32 + di;
                        let nj = j as i32 + dj;

                        if ni < 0 || nj < 0 || ni >= maplen as i32 || nj >= linelen as i32 {
                            continue;
                        }

                        new_neighbours[ni as usize][nj as usize] -= 1;
                    }
                }
            }
        }
    }

    (nr_removals, new_neighbours)
}

fn parse(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .filter_map(|line| {
            if line.is_empty() {
                return None;
            }

            Some(line.chars().map(|c| c == '@').collect())
        })
        .collect()
}
