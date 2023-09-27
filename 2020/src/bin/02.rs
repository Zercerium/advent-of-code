// Day 2: Password Philosophy
fn main() {
    let file = aoc_util::read_input_file(2020, 2);
    let passwords = file.lines().map(Password::new).collect::<Vec<_>>();
    // Part 1
    let correct = passwords.iter().filter(|&p| p.is_correct()).count();
    println!("Result Part 1: {:?}", correct);

    // Part 2
    let correct = passwords.iter().filter(|&p| p.is_correct2()).count();
    println!("Result Part 2: {:?}", correct);
}

struct Password {
    min: u8,
    max: u8,
    letter: char,
    password: String,
}

impl Password {
    fn new(s: &str) -> Self {
        let mut components = s.split(' ');
        let (min, max) = components.next().unwrap().split_once('-').unwrap();
        let (min, max) = (min.parse().unwrap(), max.parse().unwrap());
        let letter = components.next().unwrap().chars().next().unwrap();
        let password = components.next().unwrap().to_string();
        Password {
            min,
            max,
            letter,
            password,
        }
    }

    fn is_correct(&self) -> bool {
        let count = self.password.chars().filter(|s| s.eq(&self.letter)).count() as u8;
        self.min <= count && count <= self.max
    }

    fn is_correct2(&self) -> bool {
        let bytes = self.password.as_bytes();
        let letter = self.letter as u8;
        let f = |n: u8| n as usize - 1;
        (bytes[f(self.min)] == letter) ^ (bytes[f(self.max)] == letter)
    }
}
