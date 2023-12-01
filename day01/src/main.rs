fn main() {
    let lines = include_str!("input.txt");
    let mut sum = 0;
    for line in lines.lines() {
        sum += (find_first_digit(line) * 10) + find_last_digit(line);
    }

    println!("{sum}");
}

fn find_first_digit(s: &str) -> u32 {
    let len = s.len();
    for (i, c) in s.char_indices() {
        if let Some(d) = find_digit_word(&s[i..len]) {
            return d;
        }
        if let Some(d) = c.to_digit(10) {
            return d;
        }
    }
    unreachable!("invalid input")
}

fn find_last_digit(s: &str) -> u32 {
    let len = s.len();
    for (i, c) in s.char_indices().rev() {
        if let Some(d) = find_digit_word(&s[i..len]) {
            return d;
        }
        if let Some(d) = c.to_digit(10) {
            return d;
        }
    }
    unreachable!("invalid input")
}

#[rustfmt::skip]
fn find_digit_word(s: &str) -> Option<u32> {
    if s.starts_with("one") { return Some(1)};
    if s.starts_with("two") { return Some(2)};
    if s.starts_with("three") { return Some(3)};
    if s.starts_with("four") { return Some(4)};
    if s.starts_with("five") { return Some(5)};
    if s.starts_with("six") { return Some(6)};
    if s.starts_with("seven") { return Some(7)};
    if s.starts_with("eight") { return Some(8)};
    if s.starts_with("nine") { return Some(9)};
    if s.starts_with("zero") { return Some(0)};

    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        // assert_eq!(find_first_digit("1abc2"), 1);
        // assert_eq!(find_last_digit("1abc2"), 2);

        // assert_eq!(find_first_digit("treb7uchet"), 7);
        // assert_eq!(find_last_digit("treb7uchet"), 7);

        // assert_eq!(find_first_digit("a1b2c3d4e5f"), 1);
        // assert_eq!(find_last_digit("a1b2c3d4e5f"), 5);

        assert_eq!(find_first_digit("two1nine"), 2);
        assert_eq!(find_last_digit("two1nine"), 9);
        assert_eq!(find_first_digit("abcone2threexyz"), 1);
        assert_eq!(find_last_digit("abcone2threexyz"), 3);
    }
}
