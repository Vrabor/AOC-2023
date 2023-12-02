fn decode_simple(s: &str) -> u32 {
    let first = s
        .chars()
        .find(|&x| x.is_ascii_digit())
        .map(|c| c.to_digit(10))
        .flatten()
        .unwrap();
    let last = s
        .chars()
        .rev()
        .find(|&x| x.is_ascii_digit())
        .map(|c| c.to_digit(10))
        .flatten()
        .unwrap();
    first * 10 as u32 + last
}

fn decode_complex(s: &str) -> u32 {
    let mut symbols = Vec::new();
    let len = s.len() + 1;
    for (i, c) in s.chars().enumerate() {
        if c.is_ascii_digit() {
            symbols.push(c)
        } else {
            if len - i > 3 {
                match &s[i..i + 3] {
                    "one" => symbols.push('1'),
                    "two" => symbols.push('2'),
                    "six" => symbols.push('6'),
                    _ => {}
                }
            }
            if len - i > 4 {
                match &s[i..i + 4] {
                    "four" => symbols.push('4'),
                    "five" => symbols.push('5'),
                    "nine" => symbols.push('9'),
                    _ => {}
                }
            }
            if len - i > 5 {
                match &s[i..i + 5] {
                    "three" => symbols.push('3'),
                    "seven" => symbols.push('7'),
                    "eight" => symbols.push('8'),
                    _ => {}
                }
            }
        }
    }
    let first = symbols[0].to_digit(10).unwrap();
    let last = symbols[symbols.len() - 1].to_digit(10).unwrap();
    first * 10 + last
}

fn part_1(s: &str) -> u32 {
    s.lines()
        .map(|s| decode_simple(&s))
        .reduce(|acc, e| acc + e)
        .unwrap()
}

fn part_2(s: &str) -> u32 {
    s.lines()
        .map(|s| decode_complex(&s))
        .reduce(|acc, e| acc + e)
        .unwrap()
}

fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let p1 = part_1(&input);
    println!("Part1: {}", p1);
    let p2 = part_2(&input);
    println!("Part2: {}", p2);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_example_1() {
        assert_eq!(decode_simple("1abc2"), 12);
    }

    #[test]
    fn part1_example_2() {
        assert_eq!(decode_simple("pqr3stu8vwx"), 38);
    }

    #[test]
    fn part1_example_3() {
        assert_eq!(decode_simple("a1b2c3d4e5f"), 15);
    }

    #[test]
    fn part1_example_4() {
        assert_eq!(decode_simple("treb7uchet"), 77);
    }

    #[test]
    fn part2_example_1() {
        assert_eq!(decode_complex("two1nine"), 29);
    }

    #[test]
    fn part2_example_2() {
        assert_eq!(decode_complex("eightwothree"), 83);
    }

    #[test]
    fn part2_example_3() {
        assert_eq!(decode_complex("abcone2threexyz"), 13);
    }

    #[test]
    fn part2_example_4() {
        assert_eq!(decode_complex("xtwone3four"), 24);
    }

    #[test]
    fn part2_example_5() {
        assert_eq!(decode_complex("4nineeightseven2"), 42);
    }

    #[test]
    fn part2_example_6() {
        assert_eq!(decode_complex("zoneight234"), 14);
    }

    #[test]
    fn part2_example_7() {
        assert_eq!(decode_complex("7pqrstsixteen"), 76);
    }
}
