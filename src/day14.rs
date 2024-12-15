use std::{array, cmp::Ordering};

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

pub fn part1(input: &str) -> impl std::fmt::Display {
    const HALF_WIDTH: i32 = WIDTH / 2;
    const HALF_HEIGHT: i32 = HEIGHT / 2;
    const SECONDS: i32 = 100;
    
    let input = input.as_bytes();
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;

    let mut i = 0;
    loop {
        if i == input.len() {
            break;
        }
        // Parse robot
        let r = unsafe { parse_robot(input, &mut i) };
        i += 1;

        // Find final position
        let dx = r.vx * SECONDS;
        let dy = r.vy * SECONDS;

        let new_x = (r.px + dx).rem_euclid(WIDTH);
        let new_y = (r.py + dy).rem_euclid(HEIGHT);

        // Check quadrant
        match new_x.cmp(&HALF_WIDTH) {
            Ordering::Less => match new_y.cmp(&HALF_HEIGHT) {
                Ordering::Less => q2 += 1,      // top left
                Ordering::Greater => q3 += 1,   // bottom left
                Ordering::Equal => (),
            },
            Ordering::Greater => match new_y.cmp(&HALF_HEIGHT) {
                Ordering::Less => q1 += 1,      // top right
                Ordering::Greater => q4 += 1,   // bottom right
                Ordering::Equal => (),
            },
            Ordering::Equal => (),
        }
    }
    q1 * q2 * q3 * q4
}
// 218965032

pub fn part2(input: &str) -> impl std::fmt::Display {
    unsafe { inner_part2(input.as_bytes()) }
}
// 7037

pub unsafe fn inner_part2(input: &[u8]) -> i32 {
    // COUNT must be <= 500. The higher the better chance of getting the right answer. 50 seems good enough.
    const COUNT: usize = 50;
    let mut robots: [Robot; COUNT] = array::from_fn(|_| Robot::default());

    // Parse robots
    let mut i = 0;
    for r in robots.iter_mut() {
        *r = parse_robot(input, &mut i);
        i += 1;
    }

    // We only check the first 103 steps for y steps and 101 steps for x steps.
    // At each step we approximate the x and y variance separately and save the steps which had
    // the lowest variance. Then we use chinese remainder theorem to solve these equations:
    //    R = sx (mod 101)
    //    R = sy (mod 103)
    // Where R is the total number of steps to get the christmas tree, sx is the step < 101 that
    // had the lowest x-coord variance, and sy is the step < 103 that had the lowest y-coord
    // variance.

    let mut sx = 0;
    let mut sy = 0;
    let mut min_var_x = u32::MAX;
    let mut min_var_y = u32::MAX;

    for s in 1..=HEIGHT as usize {
        let mut i = 0;
        let mut prev_x = robots.get_unchecked(0).px;
        let mut prev_y = robots.get_unchecked(0).py;
        let mut tot_x = 0;
        let mut tot_y = 0;

        loop {
            let r = robots.get_unchecked_mut(i);

            // Take step
            r.px = (r.px + r.vx).rem_euclid(WIDTH);
            r.py = (r.py + r.vy).rem_euclid(HEIGHT);

            // Rough approximation of variance
            tot_x += r.px.abs_diff(prev_x);
            tot_y += r.py.abs_diff(prev_y);
            prev_x = r.px;
            prev_y = r.py;

            if i == COUNT - 1 {
                break;
            }
            i += 1;
        }

        // Update optimal step for x/y if variance was low enough
        if tot_x < min_var_x && s <= WIDTH as usize {
            min_var_x = tot_x;
            sx = s;
        }
        if tot_y < min_var_y {
            min_var_y = tot_y;
            sy = s;
        }
    }
    let sx = sx as i32;
    let sy = sy as i32;

    // We need to get to this equation:
    //   R = (fw * 101) + (fh * 103)
    // Where:
    //   103 * fh = sx (mod 101)
    //   101 * fw = sy (mod 103)
    // =>
    //    2 * fh = sx (mod 101)
    //   -2 * fw = sy (mod 103)
    // Therefore if sx is even, then we have:
    //   fh = sx / 2
    // Otherwise, since width is odd and odd + odd = even, we would have:
    //   fh = (sx + 101) / 2
    // Same applies to fw except with -2 instead of 2.

    let fh = if sx & 1 == 0 {
        (sx) / 2
    } else {
        (sx + WIDTH) / 2
    };

    let fw = if sy & 1 == 0 {
        (sy) / 2
    } else {
        (sy - HEIGHT).abs() / 2
    };

    (fw * WIDTH + fh * HEIGHT).rem_euclid(WIDTH * HEIGHT)
}

#[derive(Debug, Default)]
pub struct Robot {
    pub px: i32,
    pub py: i32,
    pub vx: i32,
    pub vy: i32,
}

#[inline(always)]
pub unsafe fn parse_robot(input: &[u8], i: &mut usize) -> Robot {
    *i += 2;

    // Get px
    let mut px = (*input.get_unchecked(*i) - b'0') as i32;
    *i += 1;
    let mut b = *input.get_unchecked(*i);
    while b != b',' {
        px = px * 10 + (b - b'0') as i32;
        *i += 1;
        b = *input.get_unchecked(*i);
    }
    *i += 1;

    // Get py
    let mut py = (*input.get_unchecked(*i) - b'0') as i32;
    *i += 1;
    b = *input.get_unchecked(*i);
    while b != b' ' {
        py = py * 10 + (b - b'0') as i32;
        *i += 1;
        b = *input.get_unchecked(*i);
    }
    *i += 3;

    // Get vx
    let sign = if *input.get_unchecked(*i) == b'-' {
        *i += 1;
        -1
    } else {
        1
    };
    let mut vx = (*input.get_unchecked(*i) - b'0') as i32;
    *i += 1;
    b = *input.get_unchecked(*i);
    while b != b',' {
        vx = vx * 10 + (b - b'0') as i32;
        *i += 1;
        b = *input.get_unchecked(*i);
    }
    vx *= sign;
    *i += 1;

    // Get vy
    let sign = if *input.get_unchecked(*i) == b'-' {
        *i += 1;
        -1
    } else {
        1
    };
    let mut vy = (*input.get_unchecked(*i) - b'0') as i32;
    *i += 1;
    b = *input.get_unchecked(*i);
    while b != b'\n' {
        vy = vy * 10 + (b - b'0') as i32;
        *i += 1;
        b = *input.get_unchecked(*i);
    }
    vy *= sign;

    Robot { px, py, vx, vy }
}
