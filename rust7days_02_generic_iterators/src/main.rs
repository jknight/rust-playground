use std::ops::AddAssign;
use std::cmp::PartialOrd;

pub struct Stepper<T> {
    curr: T,
    step: T,
    stop: T,
}

impl<T> Stepper<T> {
    pub fn new(start: T, stop: T, step: T) -> Self {
        Stepper {
            curr: start,
            stop,
            step,
        }
    }
}

impl<T> Iterator for Stepper<T> where T: AddAssign + Copy + PartialOrd {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.curr >= self.stop {
            return None;
        }
        let res = self.curr;
        self.curr += self.step;
        Some(res)
    }
}

fn sum_list<I, S>(l:I, mut s:S) -> S
    where I:Iterator<Item=S>, S:AddAssign, {
    for n in l {
        s += n;
    }
    s
}

fn main() {
    println!("Hello, world!");
}

// cargo test -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut c = 0;

        for n in Stepper::new(0,10,2) {
            println!("{}", n);
            c += n;
        }
        assert_eq!(c, 20);
    }

    #[test]
    fn it_sums() {
        assert_eq!(sum_list(Stepper::new(3,10,2), 0), 24);
    }

    #[test]
    fn it_folds() {
        let fl = Stepper::new(4,10,2).fold(0, |acc,x|acc + x);
        assert_eq!(fl, 18);
    }

}
