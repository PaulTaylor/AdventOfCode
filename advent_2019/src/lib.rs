#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn get_param_value(memory: &mut [isize], p_modes: &[char], ip: usize, offset: usize) -> isize {
    match p_modes.get(offset - 1).unwrap_or(&'0') {
        '0' => memory[memory[ip + offset] as usize], // Position Mode
        '1' => memory[ip + offset],                  // Immediate Mode
        x => panic!("unknown parameter mode {x}"),
    }
}

/// Intcode computer for Advent 2019
///
/// ip = Instruction Pointer
///
/// # Panics
///
/// Will panic if the opcode is unknown
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn run_program(memory: &mut [isize], input: &[isize]) -> Vec<isize> {
    let mut ip = 0usize;
    let mut in_it = input.iter();
    let mut output: Vec<isize> = vec![];

    while memory[ip] != 99 {
        let step;

        // Unpack the opcode
        let op_str = format!("{:02}", memory[ip]);
        let op_vec: Vec<_> = op_str.chars().collect();

        // Op is the last two chars of the op_str
        let op: usize = format!("{}{}", op_vec[op_vec.len() - 2], op_vec[op_vec.len() - 1])
            .parse()
            .unwrap();
        let p_modes: Vec<char> = op_vec.into_iter().rev().skip(2).collect();

        match op {
            // Addition
            1 => {
                let a = get_param_value(memory, &p_modes, ip, 1);
                let b = get_param_value(memory, &p_modes, ip, 2);
                let t = memory[ip + 3]; // Never in immediate mode

                memory[t as usize] = a + b;
                step = 4;
            }
            // Multiplication
            2 => {
                let a = get_param_value(memory, &p_modes, ip, 1);
                let b = get_param_value(memory, &p_modes, ip, 2);
                let t = memory[ip + 3]; // Never in immediate mode

                memory[t as usize] = a * b;
                step = 4;
            }
            // Input
            3 => {
                let t = memory[ip + 1]; // Never in immediate mode
                let v = *in_it.next().expect("an input value");
                memory[t as usize] = v;
                step = 2;
            }
            // Output
            4 => {
                let v = get_param_value(memory, &p_modes, ip, 1);
                output.push(v);
                step = 2;
            }
            // Jump-if-true
            5 => {
                let cond = get_param_value(memory, &p_modes, ip, 1);
                let t = get_param_value(memory, &p_modes, ip, 2);
                if cond == 0 {
                    step = 3; // Don't Jump if 0
                } else {
                    ip = t as usize; // jump!
                    step = 0;
                }
            }
            // jump-if-false
            6 => {
                let cond = get_param_value(memory, &p_modes, ip, 1);
                let t = get_param_value(memory, &p_modes, ip, 2);
                if cond == 0 {
                    ip = t as usize; // jump if 0
                    step = 0;
                } else {
                    step = 3; // Don't Jump
                }
            }
            // less than
            7 => {
                let a = get_param_value(memory, &p_modes, ip, 1);
                let b = get_param_value(memory, &p_modes, ip, 2);
                let t = memory[ip + 3] as usize;

                memory[t] = (a < b).into();
                step = 4;
            }
            // Equals
            8 => {
                let a = get_param_value(memory, &p_modes, ip, 1);
                let b = get_param_value(memory, &p_modes, ip, 2);
                let t = memory[ip + 3] as usize;

                memory[t] = (a == b).into();
                step = 4;
            }

            x => panic!("Unknown op code {x} at index {ip}"),
        }

        ip += step;
    }

    output
}

mod tests {
    use super::*;

    #[test]
    fn test_day02_intcode() {
        let mut memory = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        run_program(&mut memory, &[]);
        assert_eq!(memory[0], 3500);
    }

    #[test]
    fn test_day05_input() {
        let mut memory = vec![3, 0, 4, 0, 99];
        let output = run_program(&mut memory, &[50]);
        assert_eq!(memory[0], 50);
        assert_eq!(output, vec![50]);
    }

    #[test]
    fn test_day_05_pmode() {
        let mut memory = vec![1002, 4, 3, 4, 33];
        let output = run_program(&mut memory, &[]);
        assert_eq!(memory[4], 99);
    }

    #[test]
    fn test_day_05_negatives() {
        let mut memory = vec![1101, 100, -1, 4, 0];
        let output = run_program(&mut memory, &[]);
        assert_eq!(memory[4], 99);
    }

    #[test]
    fn test_day_05_eq() {
        let mut memory = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut output = run_program(&mut memory, &[8]);
        assert_eq!(output[0], 1, "Equals test failed"); // 8 == 8

        memory = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        output = run_program(&mut memory, &[7]);
        assert_eq!(output[0], 0, "Non-equals test failed"); // 7 != 8

        memory = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        output = run_program(&mut memory, &[8]);
        assert_eq!(output[0], 1, "Equals test failed"); // 8 == 8

        memory = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        output = run_program(&mut memory, &[7]);
        assert_eq!(output[0], 0, "Non-equals test failed"); // 7 != 8
    }

    #[test]
    fn test_day_05_less_than() {
        let mut memory = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut output = run_program(&mut memory, &[8]);
        assert_eq!(output[0], 0, "wrong 8 !< 8 pos mode"); // 8 not < 8

        memory = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        output = run_program(&mut memory, &[7]);
        assert_eq!(output[0], 1, "wrong 7 < 8 pos mode"); // 7 is < 8

        memory = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        output = run_program(&mut memory, &[8]);
        assert_eq!(output[0], 0, "wrong 8 !< 8 immediate mode"); // 8 not < 8

        memory = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        output = run_program(&mut memory, &[7]);
        assert_eq!(output[0], 1, "wrong 7 < 8 immediate mode"); // 7 is < 8
    }

    #[test]
    fn test_day_05_combined_test() {
        let memory = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let output = run_program(&mut memory.clone(), &[3]);
        assert_eq!(output[0], 999);
        let output = run_program(&mut memory.clone(), &[8]);
        assert_eq!(output[0], 1000);
        let output = run_program(&mut memory.clone(), &[13]);
        assert_eq!(output[0], 1001);
    }
}
