use std::fmt;
use std::fmt::Formatter;

// TODO: Refactor to package
#[derive(Debug)]
pub struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
pub struct Complex {
    real: f64,
    imag: f64
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

#[cfg(test)]
mod tests {
    use crate::display::{Complex, MinMax};

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
}