use std::collections::HashMap;

fn main() {
    let file = aoc_util::read_input_file(2020, 4);
    let sol = solver1(&file);
    println!("Result Part 1: {}", sol);
    let sol = solver2(&file);
    println!("Result Part 2: {}", sol);
}

fn solver1(s: &str) -> usize {
    let passports = parse(s);
    passports
        .iter()
        // .inspect(|p| println!("{:?}", p))
        .filter(|p| p.is_valid())
        .count()
}

fn solver2(s: &str) -> usize {
    let passports = parse(s);
    passports
        .iter()
        // .inspect(|p| println!("{:?}", p))
        .filter(|p| p.is_valid2())
        .count()
}

fn parse(s: &str) -> Vec<Passport> {
    s.split("\n\n")
        .map(|s| s.replace('\n', " "))
        .map(|s| Passport::new(&s))
        .collect::<Vec<_>>()
}

#[derive(Debug)]
struct Passport {
    fields: HashMap<String, String>,
}

impl Passport {
    fn new(s: &str) -> Self {
        let map = s
            .split(' ')
            .map(|s| s.split_once(':').unwrap())
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect::<HashMap<_, _>>();

        Passport { fields: map }
    }

    fn is_valid(&self) -> bool {
        let req_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        req_fields.iter().all(|&s| self.fields.contains_key(s))
    }

    fn is_valid2(&self) -> bool {
        self.is_valid()
            && self.validate_byr()
            && self.validate_iyr()
            && self.validate_eyr()
            && self.validate_hgt()
            && self.validate_hcl()
            && self.validate_ecl()
            && self.validate_pid()
    }

    fn validate_byr(&self) -> bool {
        let (_, s) = self.fields.get_key_value("byr").unwrap();
        if s.len() != 4 {
            return false;
        }
        if let Ok(num) = s.parse::<u16>() {
            (1920..=2002).contains(&num)
        } else {
            false
        }
    }

    fn validate_iyr(&self) -> bool {
        let (_, s) = self.fields.get_key_value("iyr").unwrap();
        if s.len() != 4 {
            return false;
        }
        if let Ok(num) = s.parse::<u16>() {
            (2010..=2020).contains(&num)
        } else {
            false
        }
    }

    fn validate_eyr(&self) -> bool {
        let (_, s) = self.fields.get_key_value("eyr").unwrap();
        if s.len() != 4 {
            return false;
        }
        if let Ok(num) = s.parse::<u16>() {
            (2020..=2030).contains(&num)
        } else {
            false
        }
    }

    fn validate_hgt(&self) -> bool {
        let (_, s) = self.fields.get_key_value("hgt").unwrap();
        let mut chars = s.chars();
        let num = chars
            .by_ref()
            .take_while(char::is_ascii_digit)
            .collect::<String>();
        let unit = s.chars().skip(num.len()).collect::<String>();
        let num: u16 = num.parse().unwrap();
        match unit.as_str() {
            "cm" => (150..=193).contains(&num),
            "in" => (59..=76).contains(&num),
            _ => false,
        }
    }

    fn validate_hcl(&self) -> bool {
        let (_, s) = self.fields.get_key_value("hcl").unwrap();
        if s.len() != 7 {
            return false;
        }
        let mut chars = s.chars();
        let first = chars.next().unwrap();
        if first != '#' {
            return false;
        }
        chars.filter(char::is_ascii_hexdigit).count() == 6
    }

    fn validate_ecl(&self) -> bool {
        let (_, s) = self.fields.get_key_value("ecl").unwrap();
        let valid = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        valid.contains(&s.as_str())
    }

    fn validate_pid(&self) -> bool {
        let (_, s) = self.fields.get_key_value("pid").unwrap();
        s.chars().filter(char::is_ascii_digit).count() == 9
    }
}

#[cfg(test)]
mod test {
    use crate::solver1;

    #[test]
    fn test() {
        let s = r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

yr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"#;
        let r = solver1(s);
        assert_eq!(r, 2);
    }
}
