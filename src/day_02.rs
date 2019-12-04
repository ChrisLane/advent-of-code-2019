use std::process::exit;

pub fn part1(input: &str) -> String {
    let program: Vec<u32> = input
        .split(",")
        .filter_map(|s| s.parse().ok())
        .collect::<Vec<_>>();

    run_program(&program, program[1], program[2])
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

pub fn part2(input: &str) -> String {
    let required_output = 19690720;
    let max_value = 99;
    let program: Vec<u32> = input
        .split(",")
        .filter_map(|s| s.parse().ok())
        .collect::<Vec<_>>();

    for a in 0..max_value + 1 {
        for b in 0..max_value + 1 {
            if run_program(&program, a, b)[0] == required_output {
                return (100 * a + b).to_string();
            }
        }
    }

    "No solution! :(".to_string()
}

fn run_program(program: &Vec<u32>, noun: u32, verb: u32) -> Vec<u32> {
    let mut pos = 0;
    let mut result: Vec<u32> = program.clone();

    result[1] = noun;
    result[2] = verb;

    loop {
        let op = result[pos];

        match op {
            1 | 2 => {
                let val1_pos = result[pos + 1] as usize;
                let val2_pos = result[pos + 2] as usize;
                let result_pos = result[pos + 3] as usize;
                let val1 = result[val1_pos];
                let val2 = result[val2_pos];
                result[result_pos] = if op == 1 {
                    val1 + val2
                } else {
                    val1 * val2
                }
            }
            99 => break,
            _ => {
                eprintln!("Unexpected operator in input");
                exit(1)
            }
        }

        pos += 4;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1("1,0,0,0,99"), "2,0,0,0,99");
        assert_eq!(part1("2,3,0,3,99"), "2,3,0,6,99");
        assert_eq!(part1("2,4,4,5,99,0"), "2,4,4,5,99,9801");
        assert_eq!(part1("1,1,1,4,99,5,6,0,99"), "30,1,1,4,2,5,6,0,99");
    }
}
