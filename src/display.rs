use std::fmt::{Display, Formatter, Result};

// TODO: Refactor to package
#[derive(Debug)]
pub struct MinMax(i64, i64);

impl Display for MinMax {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
pub struct Complex {
    real: f64,
    imag: f64
}

impl Display for Complex {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

struct List(Vec<i32>);

impl Display for List {
    fn fmt(&self, f: &mut Formatter) -> Result {
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
        let minmax = MinMax(0, 50);
        let expected = format!("({}, {})", minmax.0, minmax.1);
        assert_eq!(format!("{}", minmax), expected);
    }

    #[test]
    fn minmax_debug() {
        let minmax = MinMax(0, 50);
        let expected = format!("MinMax({}, {})", minmax.0, minmax.1);
        assert_eq!(format!("{:?}", minmax), expected);
    }

    #[test]
    fn complex_display() {
        let complex = Complex { real: 5.5, imag: -22.2 };
        let expected = format!("{} + {}i", complex.real, complex.imag);
        assert_eq!(format!("{}", complex), expected);
    }

    #[test]
    fn complex_debug() {
        let complex = Complex { real: 5.5, imag: -22.2 };
        let expected = format!("Complex {{ real: {}, imag: {} }}", complex.real, complex.imag);
        assert_eq!(format!("{:?}", complex), expected);
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