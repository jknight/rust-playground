#![warn(arithmetic_overflow)]

#[derive(Clone, Debug, Copy)]
struct MyCustomStruct {
    a: i32,
    b: u32,
    pub c: f32
}

impl MyCustomStruct {
    // The new function is a static function, and by convention a constructor
    pub fn new(a: i32, b: u32, c: f32) -> MyCustomStruct {
        MyCustomStruct {
            a: a, b: b, c: c
        }
    }

    pub fn sum(&self) -> f32 {
        self.a as f32 + self.b as f32 + self.c
    }
}



#[cfg(test)]
mod tests {

    use super::MyCustomStruct;
    use std::mem;

    #[test]
    fn basic_math_stuff() {
        assert_eq!(2+2, 4);
        assert_eq!(3.14 + 22.86, 26_f32);
        assert_eq!(2_i32.pow(2), 4);
        assert_eq!(4_f32.sqrt(), 2_f32);

        let a: u64 = 32;
        let b: u64 = 64;

        // Risky this could override
        assert_eq!(b - a, 32);

        let mut c = 100;
        c += 1;
        assert_eq!(c, 101);
    }

    #[test]
    #[should_panic]
    fn attempt_overflows() {
        let a = 10_u32;
        let b = 11_u32;
        let _ = a - b;
    }

    #[test]
    fn test_custom_struct() {
        assert_eq!(mem::size_of::<MyCustomStruct>(),
                mem::size_of::<i32>()
                + mem::size_of::<u32>()
                + mem::size_of::<f32>());

        let m = MyCustomStruct::new(1, 2, 3_f32);
        assert_eq!(m.a, 1);
        assert_eq!(m.b, 2);
        assert_eq!(m.c, 3_f32);

        assert_eq!(m.sum(), 6_f32);
        let m2 = m.clone();
        // Format on the struct equals exactly this string:
        assert_eq!(format!("{:?}", m2), "MyCustomStruct { a: 1, b: 2, c: 3.0 }");

        let mut m3 = m;
        m3.a = 100;

        assert_eq!(m2.a, 1);
        assert_eq!(m.a, 1);
        assert_eq!(m3.a, 100);
    }
}
