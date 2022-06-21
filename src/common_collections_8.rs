/// Given a list of integers, use a vector and return the median (when sorted, the value in the
/// middle position) and mode (the value that occurs most often; a hash map will be helpful here)
/// of the list.
#[allow(dead_code)]
pub mod median_and_mode {
    use std::collections::HashMap;

    pub fn median(values: &Vec<i32>) -> Option<f32> {
        if values.is_empty() {
            return None;
        }
        let mut values = values.clone();
        values.sort();

        if values.len() % 2 == 0 {
            return Some((values[values.len() / 2 - 1] + values[values.len() / 2]) as f32 / 2.0);
        }
        Some(values[values.len() / 2] as f32)
    }

    pub fn mode(values: &Vec<i32>) -> Option<i32> {
        if values.is_empty() {
            return None;
        }

        let mut map = HashMap::new();
        for value in values {
            *map.entry(value).or_insert(0) += 1;
        }
        Some(*map.into_iter().max_by_key(|(_, count)| *count).unwrap().0)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_median() {
            let values = Vec::new();
            assert_eq!(median(&values), None);

            let values = vec![1];
            assert_eq!(median(&values), Some(1.0));

            let values = vec![1, 2];
            assert_eq!(median(&values), Some(1.5));

            let values = vec![3, 2, 1];
            assert_eq!(median(&values), Some(2.0));

            let values = vec![3, 4, 1, 2];
            assert_eq!(median(&values), Some(2.5));

            let values = vec![10, 10, 2, 2, 3];
            assert_eq!(median(&values), Some(3.0));
            assert_eq!(median(&values), Some(3.0));
        }

        #[test]
        fn test_mode() {
            let values = Vec::new();
            assert_eq!(mode(&values), None);

            let values = vec![1];
            assert_eq!(mode(&values), Some(1));

            let values = vec![1, 1];
            assert_eq!(mode(&values), Some(1));

            let values = vec![1, 2, 1];
            assert_eq!(mode(&values), Some(1));
            assert_eq!(mode(&values), Some(1));

            let values = vec![1, 1, 2, 2];
            let result = mode(&values);
            assert!(result == Some(1) || result == Some(2));
        }
    }
}

/// Convert strings to pig latin. The first consonant of each word is moved to the end of the word
/// and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added
/// to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
#[allow(dead_code)]
pub mod strings_to_pig_latin {
    const VOWELS: &str = "aeiou";

    pub fn to_pig_latin(s: &str) -> String {
        let mut result = String::new();

        for word in s.split_whitespace() {
            let mut chars = word.chars();
            let first_char = chars.next().unwrap();

            if !first_char.is_ascii_alphabetic() {
                panic!("Unsupported character: {}", first_char);
            } else if VOWELS.contains(first_char.to_ascii_lowercase()) {
                result.push_str(&format!("{}-hay", word));
            } else {
                result.push_str(&format!("{}-{}ay", chars.as_str(), first_char));
            }
            result.push(' ');
        }
        result.pop();
        result
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_to_pig_latin() {
            assert_eq!(to_pig_latin(""), "");
            assert_eq!(to_pig_latin("first"), "irst-fay");
            assert_eq!(to_pig_latin("apple"), "apple-hay");

            assert_eq!(
                to_pig_latin("the quick brown fox"),
                "he-tay uick-qay rown-bay ox-fay"
            );
            assert_eq!(
                to_pig_latin("I think therefore I am"),
                "I-hay hink-tay herefore-tay I-hay am-hay"
            );
            assert_eq!(to_pig_latin("Test  two spaces"), "est-Tay wo-tay paces-say");
        }
    }
}

/// Using a hash map and vectors, create a text interface to allow a user to add employee names to a
/// department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then
/// let the user retrieve a list of all people in a department or all people in the company by
/// department, sorted alphabetically.
#[allow(dead_code)]
pub mod text_interface_to_company_department {
    use std::collections::HashMap;
    use std::io::Write;

    pub struct Company {
        departments: HashMap<String, Vec<String>>,
    }

    impl Company {
        pub fn new() -> Self {
            Self {
                departments: HashMap::new(),
            }
        }

        pub fn add_employee(&mut self, department: &str, employee: &str) {
            self.departments
                .entry(department.to_string())
                .or_insert(Vec::new())
                .push(employee.to_string());
        }

        pub fn employees_in_department(&self, department: &str) -> Vec<String> {
            let mut employees = self
                .departments
                .get(department)
                .map(|employees| employees.clone())
                .unwrap_or_else(|| Vec::new());
            employees.sort();
            employees
        }

        pub fn employees_in_company(&self) -> Vec<String> {
            let mut employees = self
                .departments
                .values()
                .flat_map(|employees_in_department| employees_in_department.clone())
                .collect::<Vec<_>>();
            employees.sort();
            employees
        }

        pub fn parse_command(&mut self, command: &str) -> bool {
            let mut words = command.split_whitespace();

            match words.next().unwrap_or_default() {
                "Add" => {
                    if let (Some(employee), Some(to), Some(department)) =
                        (words.next(), words.next(), words.next())
                    {
                        if to == "to" {
                            self.add_employee(department, employee);
                            return true;
                        }
                    }
                }
                "List" => {
                    if let Some(department) = words.next() {
                        println!("{:?}", self.employees_in_department(department));
                        return true;
                    }
                }
                "ListAll" => {
                    println!("{:?}", self.employees_in_company());
                    return true;
                }
                _ => {}
            }
            false
        }

        pub fn run(&mut self) {
            loop {
                print!("Enter command: ");
                std::io::stdout().flush().unwrap();

                let mut command = String::new();
                std::io::stdin().read_line(&mut command).unwrap();

                if command.trim() == "quit" || command.trim() == "exit" {
                    break;
                }

                if !self.parse_command(&command) {
                    println!("Unrecognized command: {}", command);
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_add_employee() {
            let mut company = Company::new();
            company.add_employee("Engineering", "Sally");
            company.add_employee("Engineering", "Alice");
            company.add_employee("Sales", "Amir");
            company.add_employee("Sales", "Bob");
            assert_eq!(
                company.employees_in_department("Engineering"),
                vec!["Alice", "Sally"]
            );
            assert_eq!(
                company.employees_in_department("Sales"),
                vec!["Amir", "Bob"]
            );
            assert_eq!(
                company.employees_in_company(),
                vec!["Alice", "Amir", "Bob", "Sally"]
            );
        }

        #[test]
        fn test_parse_command() {
            let mut company = Company::new();
            assert!(company.parse_command("Add Sally to Engineering"));
            assert_eq!(
                company.employees_in_department("Engineering"),
                vec!["Sally"]
            );

            assert!(company.parse_command("Add Amir to Sales"));
            assert_eq!(company.employees_in_department("Sales"), vec!["Amir"]);

            assert!(company.parse_command("List Engineering"));
            assert!(company.parse_command("ListAll"));
            assert!(!company.parse_command("List"));
            assert!(!company.parse_command("Add"));
        }
    }
}
