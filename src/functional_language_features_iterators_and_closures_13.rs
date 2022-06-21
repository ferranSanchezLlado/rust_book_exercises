/// Try modifying Cacher to hold a hash map rather than a single value. The keys of the hash map
/// will be the arg values that are passed in, and the values of the hash map will be the result of
/// calling the closure on that key. Instead of looking at whether self.value directly has a Some or
/// a None value, the value function will look up the arg in the hash map and return the value if
/// it’s present. If it’s not present, the Cacher will call the closure and save the resulting value
/// in the hash map associated with its arg value.
///
/// The second problem with the current Cacher implementation is that it only accepts closures that
/// take one parameter of type u32 and return a u32. We might want to cache the results of closures
/// that take a string slice and return usize values, for example. To fix this issue, try introducing
/// more generic parameters to increase the flexibility of the Cacher functionality.
#[allow(dead_code)]
pub mod closures {
    use std::collections::hash_map::Entry;
    use std::collections::HashMap;
    use std::hash::Hash;

    struct Cacher<T, R, F>
    where
        F: FnMut(T) -> R,
        T: Eq + Hash,
    {
        calculation: F,
        cache: HashMap<T, R>,
    }

    impl<T, R, F> Cacher<T, R, F>
    where
        F: FnMut(T) -> R,
        T: Eq + Hash + Clone,
    {
        pub fn new(calculation: F) -> Cacher<T, R, F> {
            Cacher {
                calculation,
                cache: HashMap::new(),
            }
        }

        fn value(&mut self, arg: T) -> &mut R {
            self.cache
                .entry(arg.clone())
                .or_insert_with(|| (self.calculation)(arg))
        }

        /// Implementation using match
        fn value_2(&mut self, arg: T) -> &mut R {
            match self.cache.entry(arg.clone()) {
                Entry::Occupied(entry) => entry.into_mut(),
                Entry::Vacant(entry) => entry.insert((self.calculation)(arg)),
            }
        }

        /// Implementation using if
        fn value_3(&mut self, arg: T) -> &mut R {
            if !self.cache.contains_key(&arg) {
                self.cache
                    .insert(arg.clone(), (self.calculation)(arg.clone()));
            }
            self.cache.get_mut(&arg).unwrap()
        }
    }

    struct CacherRef<'a, T, R, F>
    where
        F: FnMut(&'a T) -> R,
        T: Eq + Hash,
    {
        calculation: F,
        cache: HashMap<&'a T, R>,
    }

    impl<'a, T, R, F> CacherRef<'a, T, R, F>
    where
        F: FnMut(&'a T) -> R,
        T: Eq + Hash,
    {
        pub fn new(calculation: F) -> CacherRef<'a, T, R, F> {
            CacherRef {
                calculation,
                cache: HashMap::new(),
            }
        }

        fn value(&mut self, arg: &'a T) -> &mut R {
            self.cache
                .entry(arg)
                .or_insert_with(|| (self.calculation)(arg))
        }
        // Other implementations are omitted as they are the same as the previous one.
    }

    #[cfg(test)]
    mod tests_cacher_ref {
        use super::*;

        #[test]
        fn call_with_different_values() {
            let mut c = CacherRef::new(|&a| a);

            let _v1 = c.value(&1);
            let v2 = c.value(&2);

            assert_eq!(v2, &2);
        }

        #[test]
        fn call_with_different_types() {
            let mut c = CacherRef::new(|a: &String| a.len());

            let hello = "hello ".to_string();
            let v1 = *c.value(&hello);
            let v2 = *c.value(&hello);

            assert_eq!(v1, v2);
            assert_eq!(v1, hello.len());

            let v3 = *c.value(&"world".to_string());
            assert_ne!(v1, v3);
        }

        #[test]
        fn ensure_the_closure_is_called_only_once() {
            let mut counter = 0;
            let mut c = CacherRef::new(|&a: &u32| {
                counter += 1;
                a
            });

            let v1 = *c.value(&1);
            let v2 = *c.value(&1);

            assert_eq!(v1, v2);
            assert_eq!(counter, 1);
        }
    }

    #[cfg(test)]
    mod tests_cacher {
        use super::*;
        use paste::paste;

        macro_rules! test_functions {
            ($($function:ident),+) => {
                paste! {
                    $(
                        #[test]
                        fn [<call_with_different_values_ $function>]() {
                            let mut c = Cacher::new(|a| a);

                            let _v1 = c.$function(1);
                            let v2 = c.$function(2);

                            assert_eq!(v2, &2);
                        }

                        #[test]
                        fn [<call_with_different_types_ $function>]() {
                            let mut c = Cacher::new(|a: String| a.len());

                            let hello = "hello ".to_string();
                            let v1 = *c.$function(hello.clone());
                            let v2 = *c.$function(hello.clone());

                            assert_eq!(v1, v2);
                            assert_eq!(v1, hello.len());

                            let v3 = *c.$function("world".to_string());
                            assert_ne!(v1, v3);
                        }

                        #[test]
                        fn [<ensure_the_closure_is_called_only_once_ $function>]() {
                            let mut counter = 0;
                            let mut c = Cacher::new(|a: u32| {
                                counter += 1;
                                a
                            });

                            let v1 = *c.$function(1);
                            let v2 = *c.$function(1);

                            assert_eq!(v1, v2);
                            assert_eq!(counter, 1);
                        }
                    )+
                }
            };
        }

        test_functions!(value, value_2, value_3);
    }
}

/// Recall that the purpose of the search function is to return all lines in contents that contain
/// the query. Similar to the filter example in Listing 13-19, this code uses the filter adaptor
/// to keep only the lines that line.contains(query) returns true for. We then collect the matching
/// lines into another vector with collect. Much simpler! Feel free to make the same change to use
/// iterator methods in the search_case_insensitive function as well.
#[allow(dead_code)]
pub mod io_project {
    use std::env;
    use std::error::Error;
    use std::fs;

    pub struct Config {
        pub query: String,
        pub filename: String,
        pub case_sensitive: bool,
    }

    impl Config {
        pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
            args.next();

            let query = match args.next() {
                Some(arg) => arg,
                None => return Err("Didn't get a query string"),
            };

            let filename = match args.next() {
                Some(arg) => arg,
                None => return Err("Didn't get a file name"),
            };

            let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

            Ok(Config {
                query,
                filename,
                case_sensitive,
            })
        }
    }

    pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(config.filename)?;

        let results = if config.case_sensitive {
            search(&config.query, &contents)
        } else {
            search_case_insensitive(&config.query, &contents)
        };

        for line in results {
            println!("{}", line);
        }

        Ok(())
    }

    pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        contents
            .lines()
            .filter(|line| line.contains(query))
            .collect()
    }

    pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        contents
            .lines()
            .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
            .collect()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn case_sensitive() {
            let query = "duct";
            let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

            assert_eq!(vec!["safe, fast, productive."], search(query, contents));
        }

        #[test]
        fn case_insensitive() {
            let query = "rUsT";
            let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

            assert_eq!(
                vec!["Rust:", "Trust me."],
                search_case_insensitive(query, contents)
            );
        }
    }
}
