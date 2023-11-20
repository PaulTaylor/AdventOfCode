/// Intcode computer for Advent 2019
///
/// ip = Instruction Pointer
///
/// # Panics
///
/// Will panic if the opcode is unknown
pub fn run_program(memory: &mut [usize]) {
    let mut ip = 0usize;
    while memory[ip] != 99 {
        let mut step = 0;

        match memory[ip] {
            1 => {
                let [op, a, b, t]: [usize; 4] = memory[ip..=ip + 3].try_into().unwrap();
                memory[t] = memory[a] + memory[b];
                step = 4;
            }
            2 => {
                let [op, a, b, t]: [usize; 4] = memory[ip..=ip + 3].try_into().unwrap();
                memory[t] = memory[a] * memory[b];
                step = 4;
            }
            x => panic!("Unknown op code {x} at index {ip}"),
        }

        ip += step;
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_day02_intcode() {
        let mut memory = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        run_program(&mut memory);
        assert_eq!(memory[0], 3500);
    }
}
