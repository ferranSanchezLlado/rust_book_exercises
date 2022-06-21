/// Redundant module to allow the instantiation of the crates located in the directory
/// `./src/more_about_cargo_and_crates_io_14/add`. Allowing the interaction with the crates through
/// this module. The exercise consisted in the following:
///
/// For additional practice, add an add_two crate to this workspace in a similar way as the add_one
/// crate!
pub mod add {
    pub extern crate add_one;
    pub extern crate add_two;

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn add_one_test() {
            assert_eq!(3, add_one::add_one(2));
        }

        #[test]
        fn add_two_test() {
            assert_eq!(4, add_two::add_two(2));
        }
    }
}