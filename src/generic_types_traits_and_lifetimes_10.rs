/// We could also implement largest by having the function return a reference to a T value in the
/// slice. If we change the return type to &T instead of T, thereby changing the body of the
/// function to return a reference, we wouldn’t need the Clone or Copy trait bounds and we could
/// avoid heap allocations. Try implementing these alternate solutions on your own! If you get
/// stuck with errors having to do with lifetimes, keep reading: the “Validating References with
/// Lifetimes” section coming up will explain, but lifetimes aren’t required to solve these
/// challenges.
#[allow(dead_code)]
pub mod largest {
    pub fn largest<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    /// Extra implementation using streams
    pub fn largest_stream<T: PartialOrd>(list: &[T]) -> &T {
        list.iter()
            .reduce(|largest, item| if item > largest { item } else { largest })
            .unwrap()
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fmt::Debug;

        /// Enum Used for testing Non-Copyable types
        #[derive(Debug, PartialEq, PartialOrd)]
        enum Food {
            Apple,
            Carrot,
            Potato,
        }

        macro_rules! test_function {
            ($function:ident) => {
                let values = [1, 2, 3, 4, 5];
                assert_eq!($function(&values), &5);

                let values = [-2, 1, 3, 4, 5];
                assert_eq!($function(&values), &5);

                let values = [1.1, 5.5, 3.3, 4.4];
                assert_eq!($function(&values), &5.5);

                let values = ['d', 'a', 'b', 'c'];
                assert_eq!($function(&values), &'d');

                let values = [Food::Carrot, Food::Potato, Food::Apple];
                assert_eq!($function(&values), &Food::Potato);
            };
        }

        #[test]
        fn test_largest() {
            test_function!(largest);
        }

        #[test]
        fn test_largest_stream() {
            test_function!(largest_stream);
        }
    }
}
