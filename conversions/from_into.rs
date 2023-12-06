// from_into.rs
//
// The From trait is used for value-to-value conversions. If From is implemented
// correctly for a type, the Into trait should work conversely. You can read
// more about it at https://doc.rust-lang.org/std/convert/trait.From.html
//
// Execute `rustlings hint from_into` or use the `hint` watch subcommand for a
// hint.
// I am Done
#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

impl Default for Person {
    fn default() -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}

impl From<&str> for Person {
    fn from(s: &str) -> Person {
        let parts: Vec<&str> = s.split(',').collect();

        if parts.len() != 2 {
            // If the string does not have exactly two parts (name and age), return the default Person.
            return Person::default();
        }

        let name = parts[0].trim();
        let age_result = parts[1].trim().parse::<usize>();

        if name.is_empty() || age_result.is_err() {
            // If the name is empty or the age cannot be parsed, return the default Person.
            return Person::default();
        }

        let age = age_result.unwrap();

        Person {
            name: name.to_string(),
            age,
        }
    }
}

fn main() {
    let p1 = Person::from("Mark,20");
    let p2: Person = "Gerald,70".into();
    println!("{:?}", p1);
    println!("{:?}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }

    #[test]
    fn test_bad_convert() {
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_good_convert() {
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }

    #[test]
    fn test_bad_age() {
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_comma_and_age() {
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }


}

