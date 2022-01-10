// cargo test -- --nocapture
#[cfg(test)]
mod tests {
    #[test]
    fn conditionals() {
        let i = 2;
        if i < 2 {
            println!("i < 2");
            assert!(i < 2);
        } else if i > 2 {
            println!("i > 2");
            assert!(i > 2);
        } else {
            println!("i == 2");
            assert_eq!(i, 2);
        }
    }
    #[test]
    fn more_conditional() {
        let my_option = Some(10);
        // If let statement can do simple pattern matching
        if let Some(unpacked) = my_option {
            assert_eq!(unpacked, 10);
        }

        let mut other_option = Some(2);
        // there is also while let, which does the same thing
        while let Some(unpacked) = other_option {
            // if can also return values in assignments
            other_option = if unpacked > 0 {
                Some(unpacked - 1)
            } else {
                None
            }
        }
        assert_eq!(other_option, None)
    }

    #[test]
    fn test_loops() {
        let mut i = 42;
        let mut _broke = false;
        // a basic loop with control statements
        loop {
            i -=1;
            if i < 2 {
                _broke = true;
                break;
            } else if i > 2 {
                continue;
            }
        }
        // confirm that we hit the conditional
        assert!(_broke);

        // named loops
        'outer: loop {
            'inner: loop {
                break 'inner;
            }
            break 'outer;
        }
    }

    #[test]
    fn test_more_loops() {
        let mut iterations: u32 = 0;
        let total_squared = loop {
            iterations += 1;
            if iterations >= 10 {
                break iterations.pow(2);
            }
        };
        assert_eq!(total_squared, 100);
    }

    #[test]
    fn test_yet_more_loops() {
        let mut i = 42;
        let mut broke = false;
        // a basic loop with control statements
        loop {
            i -= 1;
            if i < 2 {
                broke = true;
                break;
            } else if i > 2 {
                continue;
            }
        }
        assert!(broke);
        let mut iterations: u32 = 0;
        let total_squared = loop {
            iterations += 1;
            if iterations >= 10 {
                break iterations.pow(2);
            }
        };
        assert_eq!(total_squared, 100);

        // playing with loops & assertions
        for i in 0..10 {
            assert!(i >= 0 && i < 10);
        }

        for v in vec![1, 1, 1, 1].iter() {
            assert_eq!(v, &1);  // note the reference on the comparison - v is a ref
        }

    }
}
