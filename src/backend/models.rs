#![allow(proc_macro_derive_resolution_fallback)]

use super::schema::tasks;
use chrono::Weekday;
use std::{fmt, str::FromStr};

#[derive(Debug, PartialEq)]
enum Recurrence {
    OneOff,
    Each(Vec<Weekday>),
}

impl fmt::Display for Recurrence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Recurrence::*;
        let s = match self {
            OneOff => "None".into(),
            Each(ds) => {
                let mut ret = String::new();
                for d in ds {
                    match d {
                        Weekday::Mon => ret.push_str("Mo"),
                        Weekday::Tue => ret.push_str("Tu"),
                        Weekday::Wed => ret.push_str("We"),
                        Weekday::Thu => ret.push_str("Th"),
                        Weekday::Fri => ret.push_str("Fr"),
                        Weekday::Sat => ret.push_str("Sa"),
                        Weekday::Sun => ret.push_str("Su"),
                    }
                }
                ret
            }
        };
        write!(f, "{}", s)
    }
}

impl FromStr for Recurrence {
    type Err = ::std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::Recurrence::*;
        if s == "None" {
            Ok(OneOff)
        } else {
            let len = s.len();
            if len > 14 {
                return Err(::std::io::Error::new(
                    ::std::io::ErrorKind::InvalidInput,
                    "wrong length recurrence data",
                ));
            } else if len % 2 != 0 {
                return Err(::std::io::Error::new(
                    ::std::io::ErrorKind::InvalidInput,
                    "wrong length recurrence data",
                ));
            }
            let mut ret = Vec::new();

            let mut chars = s.chars();
            // FIRST check if it's a duplicate
            // each branch with  call chars.next().unwrap()
            // we know it will succeed because we already threw out odd-length strs
            match chars.next() {
                Some('M') => {
                    ret.push(Weekday::Mon);
                    chars.next().unwrap();
                }
                Some('T') => {
                    if chars.next().unwrap() == 'u' {
                        ret.push(Weekday::Tue);
                    } else if chars.next().unwrap() == 'h' {
                        ret.push(Weekday::Thu);
                    } else {
                        return Err(::std::io::Error::new(
                            ::std::io::ErrorKind::InvalidInput,
                            "T followed by somethong other than u or h",
                        ));
                    }
                }
                Some('W') => {
                    ret.push(Weekday::Wed);
                    chars.next().unwrap();
                }
                Some('F') => {
                    ret.push(Weekday::Fri);
                    chars.next().unwrap();
                }
                Some('S') => {
                    if chars.next().unwrap() == 'a' {
                        ret.push(Weekday::Sat);
                    } else if chars.next().unwrap() == 'u' {
                        ret.push(Weekday::Sun);
                    } else {
                        return Err(::std::io::Error::new(
                            ::std::io::ErrorKind::InvalidInput,
                            "S followed by somethong other than a or u",
                        ));
                    }
                }
                Some(_) => {
                    return Err(::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidInput,
                        "Garbage fond in recurrence string",
                    ))
                }
                None => {
                    return Err(::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidInput,
                        "Wrong length recurrence string",
                    ))
                }
            }

            Ok(Each(ret))
        }
    }
}

// maybe rename DbTask, which will have a From<Task>, vica versa?
#[derive(Queryable, Serialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub completed: bool,
    pub recurrence: String, // user ??
}

#[derive(Serialize)]
pub struct TaskList {
    pub tasks: Vec<Task>,
}

#[derive(Insertable)]
#[table_name = "tasks"]
pub struct NewTask<'a> {
    pub id: &'a str,
    pub title: &'a str,
    pub completed: bool,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_recurrence_from_valid_str_none() {
        assert_eq!(Recurrence::from_str("None").unwrap(), Recurrence::OneOff)
    }
    #[test]
    fn test_recurrence_from_valid_str_single() {
        assert_eq!(
            Recurrence::from_str("Mo").unwrap(),
            Recurrence::Each(vec![Weekday::Mon])
        )
    }
    #[test]
    fn test_recurrence_from_valid_str_all_days() {
        assert_eq!(
            Recurrence::from_str("MoTuWeThFrSaSu").unwrap(),
            Recurrence::Each(vec![
                Weekday::Mon,
                Weekday::Tue,
                Weekday::Wed,
                Weekday::Thu,
                Weekday::Fri,
                Weekday::Sat,
                Weekday::Sun
            ])
        )
    }
    #[test]
    #[should_panic]
    fn test_recurrence_from_valid_str_duplicate() {
        assert_eq!(
            Recurrence::from_str("MoMo").unwrap(),
            Recurrence::Each(vec![Weekday::Mon, Weekday::Mon])
        )
    }
    #[test]
    #[should_panic]
    fn test_recurrence_from_invalid_str_odd() {
        assert_eq!(
            Recurrence::from_str("MoT").unwrap(),
            Recurrence::Each(vec![Weekday::Mon])
        )
    }
    #[test]
    #[should_panic]
    fn test_recurrence_from_invalid_str_len() {
        assert_eq!(
            Recurrence::from_str("MoTuWeThFrSaSuMo").unwrap(),
            Recurrence::Each(vec![Weekday::Mon, Weekday::Tue])
        )
    }
    #[test]
    #[should_panic]
    fn test_recurrence_from_invalid_str_garbage() {
        assert_eq!(
            Recurrence::from_str("DoReMiFaSo").unwrap(),
            Recurrence::Each(vec![Weekday::Mon])
        )
    }
    #[test]
    #[should_panic]
    fn test_recurrence_from_invalid_str_bad_t() {
        assert_eq!(
            Recurrence::from_str("MoTlWe").unwrap(),
            Recurrence::Each(vec![Weekday::Mon])
        )
    }
    #[test]
    #[should_panic]
    fn test_recurrence_from_invalid_str_bad_s() {
        assert_eq!(
            Recurrence::from_str("SaSyTh").unwrap(),
            Recurrence::Each(vec![Weekday::Mon])
        )
    }
}
