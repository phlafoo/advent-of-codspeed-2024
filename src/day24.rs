use itertools::Itertools;
use core::str;
use std::array;

pub fn part1(input: &str) -> u64 {
    unsafe { inner_part1(input.as_bytes()) }
}
// 57588078076750

pub fn part2(input: &str) -> String {
    unsafe { inner_part2(input.as_bytes()) }
}
// kcd,pfn,shj,tpk,wkb,z07,z23,z27

const SIZE: usize = get_index("www".as_bytes()) as usize;
// const SIZE: usize = get_index("z46".as_bytes()) as usize;

unsafe fn inner_part1(input: &[u8]) -> u64 {
    let mut gate_map = [const { Vec::new() }; SIZE];
    let mut gates = [const { None }; 256];
    let mut gates_len = 0;

    // Find start of gates
    let mut i = 0;
    loop {
        if &input[i..i + 2] == "\n\n".as_bytes() {
            break;
        }
        i += 1;
    }

    // Parse gates
    for line in input[i + 2..input.len() - 1].split(|&b| b == b'\n') {
        let k = line.len() - 10; // index of start of rhs gate input
        
        let gate_type = match line[4] {
            b'A' => GateType::And,
            b'O' => GateType::Or,
            b'X' => GateType::Xor,
            _ => unreachable!(),
        };

        // Save gate for later
        let a = get_index(&line[..3]);
        let b = get_index(&line[k..k + 3]);
        let out = get_index(&line[k + 7..]);

        let gate = Gate1 {
            gate_type,
            a: GateInput::new(a, None),
            b: GateInput::new(b, None),
            out,
        };

        gates[gates_len] = Some(gate);
        gate_map[a as usize].push(gates_len as u8);
        gate_map[b as usize].push(gates_len as u8);
        gates_len += 1;
    }

    let mut result = 0;

    // Iterate xy inputs
    for line in input[..i].split(|&b| b == b'\n') {
        let wire = get_index(&line[..3]);
        let sig = line[5] == b'1';

        // Iterate indices of gates that have this wire as input
        for &i in &gate_map[wire as usize] {
            let g = gates[i as usize].as_mut().unwrap();
            g.update_signal(wire, sig);

            if g.is_ready() {
                // If both inputs have been received, propogate signal
                propogate_signal(i as usize, &mut gates, &gate_map, &mut result);
            }
        }
    }
    result
}

/// Recursive function to propogate output signals
unsafe fn propogate_signal(
    i: usize,
    gates: &mut [Option<Gate1>; 256],
    gate_map: &[Vec<u8>; SIZE],
    output: &mut u64,
) {
    let source_gate = gates[i].as_ref().unwrap();
    let out = source_gate.out;
    let sig = source_gate.get_output_signal();

    let gate_indices = &gate_map[out as usize];

    if gate_indices.is_empty() {
        // Only z wires are not inputs to any gates, so we can save to result here
        *output |= (sig as u64) << source_gate.get_z_pos();
        return;
    };
    // Iterate all gates that have the source output as innput
    for &gi in gate_indices {
        let gate = gates[gi as usize].as_mut().unwrap();
        gate.update_signal(out, sig);

        if gate.is_ready() {
            // If both inputs have been received, propogate signal
            propogate_signal(gi as usize, gates, gate_map, output);
        }
    }
}

/// Turn wire name into an index for `gate_map`
const fn get_index(bytes: &[u8]) -> u16 {
    let d0 = (bytes[0] - b'a') as u16;
    let d1 = if bytes[1] >= b'a' {
        (bytes[1] - b'a') as u16
    } else {
        (bytes[1] - b'0') as u16
    };
    let d2 = if bytes[2] >= b'a' {
        (bytes[2] - b'a') as u16
    } else {
        (bytes[2] - b'0') as u16
    };
    // Reversing order makes for slightly lower max index
    d2 << 10 | d1 << 5 | d0
}

/// Get back wire name from index (for z-- names the numbers are mapped to letters, so z01 becomes zab)
#[allow(unused)]
unsafe fn get_wire_name(i: u16) -> String {
    const MASK: u8 = 0b1_1111;
    let c0 = (i >> 10) as u8 + b'a';
    let c1 = ((i >> 5) as u8 & MASK) + b'a';
    let c2 = (i as u8 & MASK) + b'a';
    core::str::from_utf8_unchecked([c2, c1, c0].as_slice()).to_string()
}

type Signal = Option<bool>;

#[derive(Default, Copy, Clone)]
struct GateInput {
    map_index: u16,
    signal: Signal,
}

impl GateInput {
    fn new(map_index: u16, signal: Signal) -> Self {
        GateInput {
            map_index,
            signal,
        }
    }
}

#[derive(Default, Clone)]
struct Gate1 {
    gate_type: GateType,
    a: GateInput,
    b: GateInput,
    out: u16,
}

impl Gate1 {
    /// Panics if either input signal is not set
    fn get_output_signal(&self) -> bool {
        match self.gate_type {
            GateType::And => self.a.signal.unwrap() && self.b.signal.unwrap(),
            GateType::Or => self.a.signal.unwrap() || self.b.signal.unwrap(),
            GateType::Xor => self.a.signal.unwrap() ^ self.b.signal.unwrap(),
        }
    }

    /// Assumes that no gate has the same `a` and `b` input 
    fn update_signal(&mut self, map_index: u16, signal: bool) {
        if self.a.map_index == map_index {
            self.a.signal = Some(signal);
        } else if self.b.map_index == map_index {
            self.b.signal = Some(signal);
        }
    }

    /// Returns true if both inputs have been received
    fn is_ready(&self) -> bool {
        self.a.signal.is_some() && self.b.signal.is_some()
    }

    /// E.g. A wire name of z36 would return 36
    fn get_z_pos(&self) -> u64 {
        const MASK: u16 = 0b1_1111;
        let d0 = ((self.out >> 5) & MASK) as u64;
        let d1 = ((self.out >> 10) & MASK) as u64;
        d0 * 10 + d1
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum GateType {
    #[default]
    And,
    Or,
    Xor,
}



unsafe fn inner_part2(input: &[u8]) -> String {
    let mut gate_map = [const { Vec::new() }; SIZE];

    let mut gates: [Gate2; 180] = array::from_fn(|_| Gate2::default());
    // let mut gates = [const { None }; 180];

    let mut gates_len = 0;

    // Find start of gates
    let mut i = 0;
    loop {
        if &input[i..i + 2] == "\n\n".as_bytes() {
            break;
        }
        i += 1;
    }

    // List of outputs that need swappin
    let mut sussy = vec![];

    /*
    Sus conditions:
    1. Non-XOR gate that outputs z.
    2. XOR gate that doesn't have xy inputs *and* doesn't have z output.
    3. AND gate that has xy inputs and its output goes into another AND gate.
    4. XOR gate with xy inputs whose output goes into an OR gate.

    Failing these conditions means the adder is broken and the output of that gate must be swapped.
     */

    // Parse gates
    for line in input[i + 2..input.len() - 1].split(|&b| b == b'\n') {
        let is_xy = line[0] == b'x' || line[0] == b'y';
        let k = line.len() - 10; // index of start of rhs gate input

        let gate_type = match line[4] {
            b'A' => GateType::And,
            b'O' => GateType::Or,
            b'X' => GateType::Xor,
            _ => unreachable!(),
        };

        // Check sus condition #1 and #2
        match gate_type {
            GateType::Xor => {
                if !is_xy {
                    if line[k + 7] != b'z' {
                        sussy.push(str::from_utf8_unchecked(&line[k + 7..]).to_string());
                    }
                    continue;
                }
            }
            _ => {
                if line[k + 7] == b'z' && &line[k + 8..] != "45".as_bytes() {
                    sussy.push(str::from_utf8_unchecked(&line[k + 7..]).to_string());
                }
            }
        }
        // No need to store rhs input
        let a = get_index(&line[..3]);
        let out = get_index(&line[k + 7..]);
        let gate = Gate2 {
            gate_type,
            a,
            out,
        };
        gates[gates_len] = gate;
        gate_map[a as usize].push(gates_len as u8);
        gates_len += 1;
    }

    // Iterate over gates to check sus conditions #3 and #4
    for i in 0..gates_len {
        let gate = &gates[i];
        if !gate.has_xy_input() {
            continue;
        }
        match gate.gate_type {
            GateType::And => {
                if !gate.has_first_input_bits() {
                    for &gi in &gate_map[gate.out as usize] {
                        if gates[gi as usize].gate_type == GateType::And {
                            sussy.push(get_wire_name(gate.out));
                            break;
                        }
                    }
                }
            }
            GateType::Xor => {
                for &gi in &gate_map[gate.out as usize] {
                    if gates[gi as usize].gate_type == GateType::Or {
                        sussy.push(get_wire_name(gate.out));
                        break;
                    }
                }
            }
            GateType::Or => (),
        }
    }
    sussy.sort();
    sussy.iter().join(",")
}

#[derive(Default, Clone)]
struct Gate2 {
    gate_type: GateType,
    a: u16,
    out: u16,
}

impl Gate2 {
    fn has_xy_input(&self) -> bool {
        const MASK: u8 = 0b1_1111;
        [b'x', b'y'].contains(&((self.a as u8 & MASK) + b'a'))
    }
    /// Returns true if input has x00 or y00 as inputs
    fn has_first_input_bits(&self) -> bool {
        (self.a >> 5) == 0
    }
}