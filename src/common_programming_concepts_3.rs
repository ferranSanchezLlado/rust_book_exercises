/// Convert temperatures between Fahrenheit and Celsius.
#[allow(dead_code)]
pub mod temperature_convertor {

    pub enum Temperature {
        Kelvin, // Base unit of temperature is Kelvin
        Celsius,
        Fahrenheit,
    }

    impl Temperature {
        fn cast_to_kelvin(&self, value: f32) -> f32 {
            match self {
                Temperature::Kelvin => value,
                Temperature::Celsius => value + 273.15,
                Temperature::Fahrenheit => 5.0 / 9.0 * (value + 459.67),
            }
        }

        fn cast_from_kelvin(&self, value: f32) -> f32 {
            match self {
                Temperature::Kelvin => value,
                Temperature::Celsius => value - 273.15,
                Temperature::Fahrenheit => value * 9.0 / 5.0 - 459.67,
            }
        }

        pub fn convert(&self, value: f32, to: &Temperature) -> f32 {
            if self.cast_to_kelvin(value) < 0.0 {
                panic!("Temperature cannot be below absolute zero!");
            }
            to.cast_from_kelvin(self.cast_to_kelvin(value))
        }
    }

    #[cfg(test)]
    mod tests {
        use super::Temperature::*;

        #[test]
        fn test_kelvin_to_celsius() {
            assert_eq!(Kelvin.convert(0.0, &Celsius), -273.15);
            assert_eq!(Kelvin.convert(273.15, &Celsius), 0.0);
            assert_eq!(Kelvin.convert(373.15, &Celsius), 100.0);
        }

        #[test]
        fn test_kelvin_to_fahrenheit() {
            assert_eq!(Kelvin.convert(0.0, &Fahrenheit), -459.67);
            assert_eq!(Kelvin.convert(255.37222, &Fahrenheit), 0.0);
            assert_eq!(Kelvin.convert(310.9278, &Fahrenheit), 100.00003);
        }

        #[test]
        fn test_kelvin_to_kelvin() {
            assert_eq!(Kelvin.convert(0.0, &Kelvin), 0.0);
            assert_eq!(Kelvin.convert(100.0, &Kelvin), 100.0);
        }

        #[test]
        fn test_celsius_to_kelvin() {
            assert_eq!(Celsius.convert(0.0, &Kelvin), 273.15);
            assert_eq!(Celsius.convert(-273.15, &Kelvin), 0.0);
            assert_eq!(Celsius.convert(373.15, &Kelvin), 646.3);
        }

        #[test]
        fn test_celsius_to_fahrenheit() {
            // Precision error
            assert_eq!(Celsius.convert(0.0, &Fahrenheit), 31.99997);
            assert_eq!(Celsius.convert(-17.77778, &Fahrenheit), -3.0517578e-5);
            assert_eq!(Celsius.convert(100.0, &Fahrenheit), 211.99997);
        }

        #[test]
        fn test_celsius_to_celsius() {
            assert_eq!(Celsius.convert(0.0, &Celsius), 0.0);
            assert_eq!(Celsius.convert(100.0, &Celsius), 100.0);
        }

        #[test]
        fn test_fahrenheit_to_kelvin() {
            assert_eq!(Fahrenheit.convert(0.0, &Kelvin), 255.37224);
            assert_eq!(Fahrenheit.convert(-459.67, &Kelvin), 0.0);
            assert_eq!(Fahrenheit.convert(1000.0, &Kelvin), 810.92786);
        }

        #[test]
        fn test_fahrenheit_to_celsius() {
            assert_eq!(Fahrenheit.convert(0.0, &Celsius), -17.777756);
            assert_eq!(Fahrenheit.convert(32.0, &Celsius), 3.0517578e-5);
            assert_eq!(Fahrenheit.convert(100.0, &Celsius), 37.777832);
        }

        #[test]
        fn test_fahrenheit_to_fahrenheit() {
            assert_eq!(Fahrenheit.convert(0.0, &Fahrenheit), 0.0);
            assert_eq!(Fahrenheit.convert(100.0, &Fahrenheit), 100.00003);
        }

        #[test]
        #[should_panic(expected = "Temperature cannot be below absolute zero!")]
        fn test_kelvin_to_celsius_below_absolute_zero() {
            Kelvin.convert(-100.0, &Celsius);
        }
    }
}

/// Generate the nth Fibonacci number.
#[allow(dead_code)]
pub mod fibonacci {
    pub fn recursive(n: u32) -> u32 {
        match n {
            0 => 0,
            1 => 1,
            _ => recursive(n - 1) + recursive(n - 2),
        }
    }

    pub fn iterative(n: u32) -> u32 {
        let mut a = 0;
        let mut b = 1;
        for _ in 0..n {
            (a, b) = (b, a + b);
        }
        a
    }

    pub fn iterative_storing(n: u32) -> u32 {
        let mut values = vec![0, 1];
        for _ in 1..n {
            values.push(values[values.len() - 1] + values[values.len() - 2]);
        }
        values[n as usize]
    }

    pub fn stream(n: u32) -> u32 {
        (0..n).fold((0, 1), |(a, b), _| (b, a + b)).0
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const VALUES: [u32; 20] = [
            0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181,
        ];

        fn test_first_20(function: fn(u32) -> u32) {
            for (i, &value) in VALUES.iter().enumerate() {
                assert_eq!(function(i as u32), value);
            }
        }

        #[test]
        fn test_recursive() {
            test_first_20(recursive);
        }

        #[test]
        fn test_iterative() {
            test_first_20(iterative);
        }

        #[test]
        fn test_iterative_storing() {
            test_first_20(iterative_storing);
        }

        #[test]
        fn test_stream() {
            test_first_20(stream);
        }
    }
}

/// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the
/// repetition in the song.
#[allow(dead_code)]
pub mod the_twelve_days_of_christmas {
    // Based on lyrics from: https://genius.com/Christmas-songs-the-twelve-days-of-christmas-lyrics

    fn gift(n: u32) -> &'static str {
        match n {
            1 => "A partridge in a pear tree",
            2 => "Two turtle doves, and",
            3 => "Three french hens",
            4 => "Four calling birds",
            5 => "Five golden rings",
            6 => "Six geese a-laying",
            7 => "Seven swans a-swimming",
            8 => "Eight maids a-milking",
            9 => "Nine ladies dancing",
            10 => "Ten lords a-leaping",
            11 => "Eleven pipers piping",
            12 => "Twelve drummers drumming",
            _ => panic!("Invalid day"),
        }
    }

    fn day(n: u32) -> &'static str {
        match n {
            1 => "first",
            2 => "second",
            3 => "third",
            4 => "fourth",
            5 => "fifth",
            6 => "sixth",
            7 => "seventh",
            8 => "eighth",
            9 => "ninth",
            10 => "tenth",
            11 => "eleventh",
            12 => "twelfth",
            _ => panic!("Invalid day"),
        }
    }

    fn gifts(n: u32) -> String {
        (1..=n)
            .rev()
            .map(|i| gift(i))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn verse(n: u32) -> String {
        "On the ".to_owned()
            + day(n)
            + " day of Christmas, my true love sent to me\n"
            + &gifts(n)
            + ".\n"
    }

    pub fn song() -> String {
        (1..=12)
            .map(|i| verse(i))
            .collect::<Vec<_>>()
            .join("\n")
            .trim_end()
            .to_string()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_song() {
            let lyrics = "\
On the first day of Christmas, my true love sent to me
A partridge in a pear tree.

On the second day of Christmas, my true love sent to me
Two turtle doves, and
A partridge in a pear tree.

On the third day of Christmas, my true love sent to me
Three french hens
Two turtle doves, and
A partridge in a pear tree.

On the fourth day of Christmas, my true love sent to me
Four calling birds
Three french hens
Two turtle doves, and
A partridge in a pear tree.

On the fifth day of Christmas, my true love sent to me
Five golden rings
Four calling birds
Three french hens
Two turtle doves, and
A partridge in a pear tree.

On the sixth day of Christmas, my true love sent to me
Six geese a-laying
Five golden rings
Four calling birds
Three french hens
Two turtle doves, and
A partridge in a pear tree.

On the seventh day of Christmas, my true love sent to me
Seven swans a-swimming
Six geese a-laying
Five golden rings
Four calling birds
Three french hens
Two turtle doves, and
A partridge in a pear tree.

On the eighth day of Christmas, my true love sent to me
Eight maids a-milking
Seven swans a-swimming
Six geese a-laying
Five golden rings
Four calling birds
Three french hens
Two turtle doves, and
A partridge in a pear tree.

On the ninth day of Christmas, my true love sent to me
Nine ladies dancing
Eight maids a-milking
Seven swans a-swimming
Six geese a-laying
Five golden rings
Four calling birds
Three french hens
Two turtle doves, and
A partridge in a pear tree.

On the tenth day of Christmas, my true love sent to me
Ten lords a-leaping
Nine ladies dancing
Eight maids a-milking
Seven swans a-swimming
Six geese a-laying
Five golden rings
Four calling birds
Three french hens
Two turtle doves, and
A partridge in a pear tree.

On the eleventh day of Christmas, my true love sent to me
Eleven pipers piping
Ten lords a-leaping
Nine ladies dancing
Eight maids a-milking
Seven swans a-swimming
Six geese a-laying
Five golden rings
Four calling birds
Three french hens
Two turtle doves, and
A partridge in a pear tree.

On the twelfth day of Christmas, my true love sent to me
Twelve drummers drumming
Eleven pipers piping
Ten lords a-leaping
Nine ladies dancing
Eight maids a-milking
Seven swans a-swimming
Six geese a-laying
Five golden rings
Four calling birds
Three french hens
Two turtle doves, and
A partridge in a pear tree."
                .to_string();
            assert_eq!(lyrics, song());
        }
    }
}
