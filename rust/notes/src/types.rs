pub fn types() {
    let decimal = 65.2412f32;

    // Cannot assign with non matching type
    //let integer: u8 = decimal;

    // Cast into the type
    let integer: u8 = decimal as u8;
    let character: char = integer as char;

    println!("{} -> {} -> {}", decimal, integer, character);

    // Some casts are not permitted
    //let character: char = decimal as char;

    // type aliasing, must be UpperCamelCase
    type NanoSecond = u64;

    // From trait allows conversion between types
    struct Number {
        val: i32,
    }
    impl std::convert::From<i32> for Number {
        fn from(val: i32) -> Self {
            Number { val }
        }
    }

    let n = Number::from(-233);
    let m: Number = 34.into();

    println!("{} {}", n.val, m.val);

    #[derive(Debug, PartialEq)]
    struct EvenNumber(i32);

    // TryFrom and TryInto
    impl std::convert::TryFrom<i32> for EvenNumber {
        type Error = ();

        fn try_from(v: i32) -> Result<Self, Self::Error> {
            if v % 2 == 0 {
                Ok(EvenNumber(v))
            } else {
                Err(())
            }
        }
    }

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(3), Err(()));

    let r: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(r, Ok(EvenNumber(8)));
    let r: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(r, Err(()));

    struct Circle {
        radius: u8,
    }

    impl std::fmt::Display for Circle {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Circle with radius {}", self.radius)
        }
    }

    let c = Circle { radius: 5 };
    println!("{}", c);

    let parse5: u8 = "5".parse().unwrap();
    let turbo5 = "5".parse::<u8>().unwrap();
    println!("{}", turbo5 + parse5);

    // Blocks as expressions
    // No semicolon on the return
    let y = {
        let n = 3;
        n * (5 + 2) // y = 21
    };

    println!("{}", y);
}
