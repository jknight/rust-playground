use printer::pretty_print_pi_approx;
mod rounding;

fn main() {
    println!("Hello, world!");
    pretty_print_pi_approx(100_000);
}

mod printer {
    use rust_pilib::monte_carlo_pi;

    //internal crates can always be imported usin the crate prefix
    use crate::rounding::round;
    pub fn pretty_print_pi_approx(iterations: usize) {
        let pi = monte_carlo_pi(iterations);
        let places: usize = 2;
        println!("Pi is - {} and rounded to {} places {}", pi, places, round(pi,places));
    }

}
