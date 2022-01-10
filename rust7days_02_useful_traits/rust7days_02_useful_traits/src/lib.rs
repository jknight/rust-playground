use std::fmt::Display;
use std::fmt;

#[derive(Debug)]
pub struct USD(i32);

impl Display for USD {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        let r = (self.0 as f32) / 100.;
        if r < 0. {
            return write!(f, "-${:.2}", -r);
        }
        write!(f, "${:.2}", r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let u = USD(230);
        // We get the to_string from Display
        assert_eq!(u.to_string(), "$2.30".to_string());
    }
}
