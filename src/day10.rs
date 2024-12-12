pub fn part1(input: &str) -> impl std::fmt::Display {
    let dim = input.find('\n').unwrap() + 1;
    let input = input.as_bytes();

    let mut visited = vec![false; dim * (dim - 1)];

    let north = -(dim as i32);
    let south = dim as i32;
    let west = -1;
    let east = 1;

    let neighbors = [north, south, west, east];

    let mut stack = vec![];
    let mut result = 0;

    for (i, &b) in input.iter().enumerate() {
        // Only want to find 9s
        if b != b'9' {
            continue;
        }
        stack.push(i);
        visited.fill(false);

        // Work thru all paths starting from this 9
        while let Some(s) = stack.pop() {
            visited[s] = true;
            for nx in neighbors {
                let n = s as i32 + nx;
                // Bounds check
                if n < 0 {
                    continue;
                }
                let n = n as usize;
                if n >= input.len() || visited[n] {
                    continue;
                }
                if input[n] == input[s] - 1 {
                    if input[n] == b'0' {
                        visited[n] = true;
                        result += 1;
                        continue;
                    }
                    stack.push(n);
                }
            }
        }
    }

    result
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let dim = input.find('\n').unwrap() + 1;
    let input = input.as_bytes();

    let north = -(dim as i32);
    let south = dim as i32;
    let west = -1;
    let east = 1;

    let neighbors = [north, south, west, east];

    let mut stack = vec![];
    let mut result = 0;

    for (i, &b) in input.iter().enumerate() {
        // Only want to find 9s
        if b != b'9' {
            continue;
        }
        stack.push(i);

        // Work thru all paths starting from this 9
        while let Some(s) = stack.pop() {
            for nx in neighbors {
                let n = s as i32 + nx;
                // Bounds check
                if n < 0 || n as usize >= input.len() {
                    continue;
                }
                let n = n as usize;
                if input[n] == input[s] - 1 {
                    if input[n] == b'0' {
                        result += 1;
                        continue;
                    }
                    stack.push(n);
                }
            }
        }
    }

    result
}