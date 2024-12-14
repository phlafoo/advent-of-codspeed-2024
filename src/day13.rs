pub fn part1(input: &str) -> impl std::fmt::Display {
    let input = input.as_bytes();
    let mut result = 0;
    
    let mut i = 0;    
    loop {
        if i == input.len() + 1 {
            break;
        }
        let m = parse_machine_p1(input, &mut i);
        i += 2;

        // This formula only works if vectors A and B are not integer multiples of each other.
        // `a` and `b` must be integers, so we multiply by 10 in the formula, then check that there
        // is no remainder when dividing by 10, which tells us that the value will be an integer.
        let mut b = (m.ax * m.cy - m.ay * m.cx) * 10 / (m.ax * m.by - m.ay * m.bx);

        // `a` and `b` cannot be > 100
        if !(0..=1000).contains(&b) || b % 10 != 0 {
            continue;
        }
        b /= 10;
        let a = (m.cx - b * m.bx) * 10 / m.ax;
        if !(0..=1000).contains(&a) || a % 10 != 0 {
            continue;
        }
        result += (a / 10) * 3 + b;
    }

    result
}
// 35729

#[derive(Debug)]
struct MachineP1 {
    ax: i32,
    ay: i32,
    bx: i32,
    by: i32,
    cx: i32,
    cy: i32,
}

#[inline(always)]
fn parse_machine_p1(input: &[u8], i: &mut usize) -> MachineP1 {
    *i += 12;
    let ax = ((input[*i] - b'0') * 10 + input[*i + 1] - b'0') as i32;
    *i += 6;
    let ay = ((input[*i] - b'0') * 10 + input[*i + 1] - b'0') as i32;
    *i += 15;
    let bx = ((input[*i] - b'0') * 10 + input[*i + 1] - b'0') as i32;
    *i += 6;
    let by = ((input[*i] - b'0') * 10 + input[*i + 1] - b'0') as i32;

    *i += 12;
    let mut cx = 0;
    loop {
        cx = cx * 10 + (input[*i] - b'0') as i32;
        *i += 1;
        if input[*i] == b',' {
            break;
        }
    }
    *i += 4;
    let mut cy = 0;
    loop {
        cy = cy * 10 + (input[*i] - b'0') as i32;
        *i += 1;
        if input[*i] == b'\n' {
            break;
        }
    }
    MachineP1 {
        ax,
        ay,
        bx,
        by,
        cx,
        cy,
    }
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let input = input.as_bytes();
    let mut result = 0;
    
    let mut i = 0;    
    loop {
        if i == input.len() + 1 {
            break;
        }
        let m = parse_machine_p2(input, &mut i);
        i += 2;

        // This formula only works if vectors A and B are not integer multiples of each other.
        // `a` and `b` must be integers, so we multiply by 10 in the formula, then check that there
        // is no remainder when dividing by 10, which tells us that the value will be an integer.
        let mut b = (m.ax * m.cy - m.ay * m.cx) * 10 / (m.ax * m.by - m.ay * m.bx);

        if b < 0 || b % 10 != 0 {
            continue;
        }
        b /= 10;
        let a = (m.cx - b * m.bx) * 10 / m.ax;
        if a < 0 || a % 10 != 0 {
            continue;
        }
        result += (a / 10) * 3 + b;
    }

    result
}
// 88584689879723

#[derive(Debug)]
struct MachineP2 {
    ax: i64,
    ay: i64,
    bx: i64,
    by: i64,
    cx: i64,
    cy: i64,
}

#[inline(always)]
fn parse_machine_p2(input: &[u8], i: &mut usize) -> MachineP2 {
    *i += 12;
    let ax = ((input[*i] - b'0') * 10 + input[*i + 1] - b'0') as i64;
    *i += 6;
    let ay = ((input[*i] - b'0') * 10 + input[*i + 1] - b'0') as i64;
    *i += 15;
    let bx = ((input[*i] - b'0') * 10 + input[*i + 1] - b'0') as i64;
    *i += 6;
    let by = ((input[*i] - b'0') * 10 + input[*i + 1] - b'0') as i64;

    *i += 12;
    let mut cx = 0;
    loop {
        cx = cx * 10 + (input[*i] - b'0') as i64;
        *i += 1;
        if input[*i] == b',' {
            break;
        }
    }
    cx += 10000000000000;
    *i += 4;
    let mut cy = 0;
    loop {
        cy = cy * 10 + (input[*i] - b'0') as i64;
        *i += 1;
        if input[*i] == b'\n' {
            break;
        }
    }
    cy += 10000000000000;

    MachineP2 {
        ax,
        ay,
        bx,
        by,
        cx,
        cy,
    }
}