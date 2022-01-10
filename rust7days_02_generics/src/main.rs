#[derive(PartialEq, Debug, Clone)]
pub struct USD(i32);
#[derive(PartialEq, Debug, Clone)]
pub struct GBP(i32);
#[derive(PartialEq, Debug, Clone)]
pub struct CAD(i32);

// Convert some F to USD
pub trait ToUSDv<F> {
    // implement trait on an exchange
    // returns a float
    fn to_uv(&self, f: F) -> f32;
}

// If this if converting from a USD,
// what is the f32 ?
pub trait FromUSDv<F> {
    fn from_uv(&self, f: f32) -> F;
}

pub struct Ex {
    ac_id: i32,
    cad: f32,
    gbp: f32,
}

impl Account for Ex {
    fn id(&self) -> i32 {
        self.ac_id
    }
}

#[derive(PartialEq, Debug)]
pub struct Transaction<A> {
    from_id: i32,
    to_id: i32,
    // The amount can be of different types
    amount: A,
}

// Implement traits for the exchange
impl ToUSDv<GBP> for Ex {
    fn to_uv(&self, g: GBP) -> f32 {
        (g.0 as f32) * self.gbp
    }
}

pub trait Account {
    fn id(&self) -> i32;
}

pub trait Exchange<F, T> {
    fn convert(&self, f: F) -> T;
}

// Implement an exchange for a generic
//  type
impl<E, F, T> Exchange<F, T> for E
where
    E: ToUSDv<F> + FromUSDv<T>,
{
    fn convert(&self, f: F) -> T {
        self.from_uv(self.to_uv(f))
    }
}

impl FromUSDv<CAD> for Ex {
    //  why does this take an f32 but
    //  ToUsdv takes a GBP ?
    fn from_uv(&self, f: f32) -> CAD {
        CAD((f / self.cad) as i32)
    }
}

pub trait ExchangeAccount<F, T> {
    fn exchange(&self, f_id: i32, t_id: i32, amount: F) -> (Transaction<F>, Transaction<T>);
}

impl<E, F, T> ExchangeAccount<F, T> for E
where
    E: Exchange<F, T> + Account,
    F: Clone,
{
    fn exchange(&self, f_id: i32, t_id: i32, amount: F) -> (Transaction<F>, Transaction<T>) {
        let ft = Transaction {
            from_id: f_id,
            to_id: self.id(),
            amount: amount.clone(),
        };
        let tt = Transaction {
            from_id: self.id(),
            to_id: t_id,
            amount: self.convert(amount),
        };
        (ft, tt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let g = GBP(200);
        let ex = Ex { ac_id: 1, cad: 0.7, gbp: 1.3 };
        // Before Exchange impl
        //let c = ex.from_uv(ex.to_uv(g));
        // After:
        let c = ex.convert(g);
        assert_eq!(c, CAD(371));
    }

    #[test]
    fn ex_test() {
        let ex = Ex {
            ac_id: 30,
            cad: 0.7,
            gbp: 1.3,
        };
        let (ft, tt) = ex.exchange(20,40,GBP(200));
        assert_eq!(ft, Transaction {
            from_id: 20,
            to_id: 30,
            amount: GBP(200),
        });
        assert_eq!(tt, Transaction {
            from_id: 30,
            to_id: 40,
            amount: CAD(371),
        });


    }
}

fn main() {
    println!("Hello, world!");
}
