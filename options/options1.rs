// options1.rs
//
// Execute `rustlings hint options1` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

// This function returns how much icecream there is left in the fridge.
// If it's before 10PM, there's 5 pieces left. At 10PM, someone eats them
// all, so there'll be no more left :(
// This function returns how much ice cream there is left in the fridge.
// If it's before 10 PM, there's 5 pieces left. At 10 PM, someone eats them
// all, so there'll be no more left :(
    fn maybe_icecream(time_of_day: u16) -> Option<u16> {
        // We use the 24-hour system here, so 10 PM is a value of 22 and 12 AM is a
        // value of 0. The Option output should gracefully handle cases where
        // time_of_day > 23.
        match time_of_day {
            0..=21 => Some(5), // Before 10 PM
            22..=23 => Some(0), // 10 PM or later
            _ => None, // Invalid input
        }
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;
    
        #[test]
        fn check_icecream() {
            assert_eq!(maybe_icecream(9), Some(5));
            assert_eq!(maybe_icecream(10), Some(5));
            assert_eq!(maybe_icecream(23), Some(0));
            assert_eq!(maybe_icecream(22), Some(0));
            assert_eq!(maybe_icecream(25), None);
        }
    
        #[test]
        fn raw_value() {
            let icecreams = maybe_icecream(12);
            assert_eq!(icecreams.unwrap(), 5);
        }
    }
    