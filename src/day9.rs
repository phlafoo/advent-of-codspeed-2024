pub fn part1(input: &str) -> impl std::fmt::Display {
    let input = input.as_bytes();
    let len = input.len();

    let mut left_id = 0;
    let mut right_id = len / 2;

    let mut left = 0; // Left index
    let mut right = len - 1; // Right index

    let mut result = 0;
    let mut pos = 0;
    let mut remaining = parse_u8(input[right]); // How many ids are remaining from the right side

    let calculate_step = |id, count, pos| -> usize { (id * count * (count + 2 * pos - 1)) / 2 };

    'outer: loop {
        if right <= left {
            // Left and right pointers have met
            // Add last remaining ids
            result += calculate_step(left_id, remaining, pos);
            break;
        }

        // Process file block from the left
        let in_left = parse_u8(input[left]);
        result += calculate_step(left_id, in_left, pos);

        // Process empty block from the left
        pos += in_left;
        left += 1;
        left_id += 1;
        let mut remaining_empty = parse_u8(input[left]);

        // Loop until the empty block is filled with files from the right
        loop {
            if right <= left {
                // Left and right pointers have met
                break 'outer;
            }
            if remaining_empty < remaining {
                let count = remaining_empty;
                remaining -= count;

                result += calculate_step(right_id, count, pos);

                // Empty block is filled, move to next file from the left
                pos += count;
                left += 1;

                break;
            } else {
                let count = remaining;
                remaining_empty -= count;

                right -= 2;
                remaining = parse_u8(input[right]);

                result += calculate_step(right_id, count, pos);
                right_id -= 1;

                // Empty block not yet filled, move the next file from the right
                pos += count;
            };
        }
    }

    result
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let input = input.as_bytes();

    // True if the file has been moved
    let mut moved = vec![false; input.len()];

    let mut left_id = 0;
    let mut pos = 0; // Index of the rearranged list of IDs
    let mut left = 0; // Left index

    let calculate_step = |id, count, pos| -> usize { (id * count * (count + 2 * pos - 1)) / 2 };

    // Indexed by file size. Indicates where in the input the leftmost file was for a given size.
    let mut cache = [input.len() - 1; 10];
    let update_cache = |cache: &mut [usize; 10], i, val| {
        for c in cache.iter_mut().take(i + 1) {
            *c = (*c).min(val);
        }
    };
    let mut result = 0;

    while left != input.len() - 1 {
        let left_size = parse_u8(input[left]);

        // Add to result if this file hasn't been moved
        if !moved[left] {
            result += calculate_step(left_id, left_size, pos);
        }
        left_id += 1;
        left += 1;
        pos += left_size;

        let mut hole = parse_u8(input[left]);
        let mut right = cache[hole]; // Start at leftmost position of last known same sized file
        let mut right_id = right / 2 + 1;

        // Fill hole
        while right > left {
            right_id -= 1;

            // Skip if moved already
            if moved[right] {
                right -= 2;
                continue;
            }
            // Get size of file
            let right_size = parse_u8(input[right]);

            // If it fits it sits
            if right_size <= hole {
                update_cache(&mut cache, hole, right);

                result += calculate_step(right_id, right_size, pos);
                pos += right_size;

                // Hole gets smaller
                hole -= right_size;
                moved[right] = true;

                if hole == 0 {
                    // Hole filled
                    break;
                }
            }
            // Hole not filled, keep searching
            right -= 2;
        }
        // If hole still wasn't filled, then there are no more files <= the hole size
        if hole != 0 {
            update_cache(&mut cache, hole, 0);
            pos += hole;
        }
        left += 1;
    }

    result
}

#[inline(always)]
fn parse_u8(b: u8) -> usize {
    (b - b'0') as usize
}
