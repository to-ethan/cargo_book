use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
struct MinMax(i64, i64);

impl Display for MinMax {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64
}

impl Display for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

struct List(Vec<i32>);

impl Display for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            // for every element except first, add a comma
            if count != 0 { write!(f,", ")?; }
            write!(f, "{}", v)?;
        }

        write!(f, "]")
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let rgb_tuple = format!("({}, {}, {})", self.red, self.green, self.blue);
        let hex_code = format!("0x{:02X}{:02X}{:02X}", self.red, self.green, self.blue);
        write!(f, "RGB {rgb} {hex}", rgb= rgb_tuple, hex= hex_code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minmax_display() {
        let m = MinMax(0, 50);
        let expected = "(0, 50)";
        assert_eq!(format!("{}", m), expected);
    }

    #[test]
    fn minmax_debug() {
        let m = MinMax(0, 50);
        let expected = "MinMax(0, 50)";
        assert_eq!(format!("{:?}", m), expected);
    }

    #[test]
    fn complex_display() {
        let n = Complex { real: 5.5, imag: -22.2 };
        let expected = "5.5 + -22.2i";
        assert_eq!(format!("{}", n), expected);
    }

    #[test]
    fn complex_debug() {
        let n = Complex { real: 5.5, imag: -22.2 };
        let expected = "Complex { real: 5.5, imag: -22.2 }";
        assert_eq!(format!("{:?}", n), expected);
    }

    #[test]
    fn int_list_display() {
        let v = List(vec![1, 2, 3, 4, 5]);
        let expected = "[1, 2, 3, 4, 5]";
        assert_eq!(format!("{}", v), expected);
    }

    // TODO: Parametrized test cases
    #[test]
    fn color_display() {
        let c = Color { red: 128, green: 255, blue: 90 };
        let expected = "RGB (128, 255, 90) 0x80FF5A";
        assert_eq!(format!("{}", c), expected);
    }

    #[test]
    fn color_display_2() {
        let c = Color { red: 0, green: 3, blue: 254 };
        let expected = "RGB (0, 3, 254) 0x0003FE";
        assert_eq!(format!("{}", c), expected);
    }

    #[test]
    fn color_display_3() {
        let c = Color { red: 0, green: 0, blue: 0 };
        let expected = "RGB (0, 0, 0) 0x000000";
        assert_eq!(format!("{}", c), expected);
    }
}