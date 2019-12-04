use std::process::exit;

pub fn part1(input: &str) -> String {
    let mut pos = 0;
    let mut program: Vec<u32> = input
        .split(",")
        .filter_map(|s| s.parse().ok())
        .collect::<Vec<_>>();

    loop {
        let op = program[pos];


        match op {
            1 | 2 => {
                let val1_pos = program[pos + 1] as usize;
                let val2_pos = program[pos + 2] as usize;
                let result_pos = program[pos + 3] as usize;
                let val1 = program[val1_pos];
                let val2 = program[val2_pos];
                program[result_pos] = if op == 1 {
                    val1 + val2
                } else {
                    val1 * val2
                }
            },
            99 => break,
            _ => {
                eprintln!("Unexpected operator in input");
                exit(1)
            }
        }

        pos += 4;
    }

    program
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

pub fn part2(input: &str) -> String {
    input.to_string()
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

    #[test]
    fn part2_test() {
        assert_eq!(part2("14"), "2");
        assert_eq!(part2("1969"), "966");
        assert_eq!(part2("100756"), "50346");
    }
}
