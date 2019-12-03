pub fn part1(input: &str) -> String {
    input
        .lines()
        .map(|s| calculate_module(s.parse().unwrap()))
        .sum::<i32>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    input
        .lines()
        .map(|s| {
            let mass = s.parse().unwrap();
            calculate_module_rec(mass, 0)
        })
        .sum::<i32>()
        .to_string()
}

fn calculate_module(mass: i32) -> i32 {
    // Floor step after division not required due to integer division
    mass / 3 - 2
}

fn calculate_module_rec(mass: i32, acc: i32) -> i32 {
    let value = mass / 3 - 2;
    if value <= 0 {
        acc
    } else {
        calculate_module_rec(value, acc + value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1("12"), "2");
        assert_eq!(part1("14"), "2");
        assert_eq!(part1("1969"), "654");
        assert_eq!(part1("100756"), "33583");
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2("14"), "2");
        assert_eq!(part2("1969"), "966");
        assert_eq!(part2("100756"), "50346");
    }
}
