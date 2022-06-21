/// Some programs allow arguments and environment variables for the same configuration. In those
/// cases, the programs decide that one or the other takes precedence. For another exercise on your
/// own, try controlling case insensitivity through either a command line argument or an environment
/// variable. Decide whether the command line argument or the environment variable should take
/// precedence if the program is run with one set to case sensitive and one set to case insensitive.
///
/// Change is implemented in [`Config::new`] to first check for the argument and then for the
/// environment variable. Also a [`test`] is created to test the functionality.
///
/// [`test`]: minigrep::tests::test_config
#[allow(dead_code)]
pub mod project {
    pub mod minigrep {
        use std::env;
        use std::error::Error;
        use std::fs;

        pub struct Config {
            pub query: String,
            pub filename: String,
            pub case_sensitive: bool,
        }

        impl Config {
            pub fn new(args: &[String]) -> Result<Config, &'static str> {
                if args.len() < 3 {
                    return Err("not enough arguments");
                }

                let query = args[1].clone();
                let filename = args[2].clone();

                let case_sensitive = if args.len() > 3 {
                    args[3] != "--case-insensitive"
                } else {
                    env::var("CASE_INSENSITIVE").is_err()
                };

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
            let mut results = Vec::new();

            for line in contents.lines() {
                if line.contains(query) {
                    results.push(line);
                }
            }

            results
        }

        pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
            let query = query.to_lowercase();
            let mut results = Vec::new();

            for line in contents.lines() {
                if line.to_lowercase().contains(&query) {
                    results.push(line);
                }
            }

            results
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

            #[test]
            fn test_config() {
                let args = vec!["minigrep", "duct", "src/main.rs"];
                let args: Vec<String> = args.iter().map(|a| a.to_string()).collect();
                let config = Config::new(&args).unwrap();
                assert_eq!(config.query, "duct");
                assert_eq!(config.filename, "src/main.rs");
                assert!(config.case_sensitive);

                let args = vec!["minigrep", "duct", "src/main.rs", "--case-insensitive"];
                let args: Vec<String> = args.iter().map(|a| a.to_string()).collect();
                let config = Config::new(&args).unwrap();
                assert!(!config.case_sensitive);

                let args = vec!["minigrep", "duct", "src/main.rs"];
                let args: Vec<String> = args.iter().map(|a| a.to_string()).collect();
                env::set_var("CASE_INSENSITIVE", "1");
                let config = Config::new(&args).unwrap();
                assert!(!config.case_sensitive);

                // Case here arg is invalid and Env var is true
                let args = vec!["minigrep", "duct", "src/main.rs", "false"];
                let args: Vec<String> = args.iter().map(|a| a.to_string()).collect();
                let config = Config::new(&args).unwrap();
                assert!(config.case_sensitive);
            }
        }
    }

    use std::env;
    use std::process;

    use minigrep::Config;

    fn main() {
        let args: Vec<String> = env::args().collect();

        let config = Config::new(&args).unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        });

        if let Err(e) = minigrep::run(config) {
            eprintln!("Application error: {}", e);

            process::exit(1);
        }
    }
}
