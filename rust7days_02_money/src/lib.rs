
// defines struct as a form of tuple
#[derive(PartialEq, Debug)]
pub struct USD (i32);
#[derive(PartialEq, Debug)]
pub struct GBP (i32);
#[derive(PartialEq, Debug)]
pub struct CAD (i32);
// others ...

// Traits are like abstract classes
pub trait ToUSD {
    fn to_usd(&self) -> USD;
    // T:FromUSD tells us that T implements FromUSD
    fn convert<T:FromUSD>(&self) -> T {
        T::from_usd(&self.to_usd())
    }
}

// Implementing trait
// "Floats do not work well for money and often
// lead to rounding errors"
impl ToUSD for GBP {
    fn to_usd(&self) -> USD {
        USD(self.0 * 130 / 100)
    }
}

pub trait FromUSD {
    // &USD: borrowed
    fn from_usd(u:&USD) -> Self;
}

impl FromUSD for CAD {
    fn from_usd(u:&USD) -> Self{
        CAD(( u.0 * 130) / 100)
    }
}


// run with `cargo test`
#[cfg(test)]
mod tests {
    // Use everything from the module that contains it
    use super::*;

    #[test]
    fn test_gbp_to_usd() {
        let g = GBP(200);
        let u = g.to_usd();
        // In order for assert_eq to work, the structs need to derive
        //  partial equality and debug
        assert_eq!(u, USD(260));
        assert_ne!(u, USD(261));
    }

    // use `cargo test -- --nocapture` to show println
    #[test]
    fn test_cad_to_usd() {
        let c = CAD::from_usd(&USD(200));
        assert_eq!(c, CAD(260));
    }

    #[test]
    fn test_trait_impl() {
        let g:GBP = GBP(200);
        // we get the method 'convert' by implementing
        // the trait
        let c:CAD = g.convert();
        assert_eq!(c, CAD(338));

    }
}
