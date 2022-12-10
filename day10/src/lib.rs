pub fn challenge_1(input: &str) -> i32 {
    let mut result = 0;

    read_commands(input, |clock, x_register| {
        if clock % 40 == 20 {
            result += clock * x_register;
        }

        clock
    });

    result
}

pub fn challenge_2(input: &str) -> String {
    let mut result = String::new();

    read_commands(input, |clock, x_register| {
        if (x_register..x_register + 3).contains(&clock) {
            result.push('#');
        } else {
            result.push('.');
        }

        if clock == 40 {
            result.push('\n');
            0
        } else {
            clock
        }
    });

    result
}

fn read_commands(input: &str, mut callback: impl FnMut(i32, i32) -> i32) {
    let mut clock = 0;
    let mut x_register = 1;

    for line in input.lines() {
        let (command, value) = line.split_once(' ').unwrap_or((line, "0"));
        let cycles = match command {
            "noop" => 1,
            "addx" => 2,
            _ => unreachable!(),
        };
        let value: i32 = value.parse().expect("malformed value");

        for _ in 0..cycles {
            clock = callback(clock + 1, x_register);
        }

        x_register += value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn it_solves_challenge_1() {
        assert_eq!(challenge_1(TEST_INPUT), 13140);
    }

    #[test]
    fn it_solves_challenge_2() {
        const OUTPUT: &str = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";

        assert_eq!(challenge_2(TEST_INPUT), OUTPUT);
    }
}
