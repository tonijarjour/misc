pub fn string_formatting() {
    println!("{}", 5 + /* 5 + */ 4);

    println!();

    // String number base formatting
    // # indicates an alternate print format
    println!("{:#b}", 255);
    println!("{:#x}", 255);
    println!("{:#o}", 255);

    println!();

    // Right justification
    println!("{:>3}", 1);
    println!("{:>3}", 10);
    println!("{:>3}", 100);

    println!();

    // Character padding
    println!("{:X<3}", "");
    println!("{:X<3}", "O");
    println!("{:X<3}", "OO");
    println!("{:X<3}", "OOO");

    println!();

    // Decimals
    println!("Pi is {:.2}", 3.14159);

    println!();

    // Debug printing
    #[derive(Debug)]
    struct Number(u8);

    println!("{:?}", Number(5));

    // Implement Display for the struct to allow normal normal printing
    impl std::fmt::Display for Number {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    println!("{}", Number(5));

    println!();

    #[derive(Debug)]
    struct Complex {
        r: f64,
        i: f64,
    }

    impl std::fmt::Display for Complex {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{} + {}i", self.r, self.i)
        }
    }

    let point = Complex { r: 5.3, i: 1.2 };
    println!("Display: {point}");
    println!("Debug: {point:?}");

    println!();

    struct List(Vec<u8>);

    impl std::fmt::Display for List {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "[")?;

            let vec = &self.0;

            for (i, v) in vec.iter().enumerate() {
                if i != 0 {
                    write!(f, ", ")?
                };
                write!(f, "{i}: {v}")?;
            }

            write!(f, "]")
        }
    }

    let l = List(vec![1, 2, 3]);
    println!("{l}");

    println!();

    struct Color {
        r: u8,
        g: u8,
        b: u8,
    }

    impl std::fmt::Display for Color {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            let Color { r, g, b } = &self;

            write!(f, "RGB ({r}, {g}, {b}) 0x{r:0>2X}{g:0>2X}{b:0>2X}")
        }
    }

    let colors = [
        Color {
            r: 255,
            g: 255,
            b: 255,
        },
        Color {
            r: 152,
            g: 34,
            b: 96,
        },
        Color { r: 0, g: 0, b: 0 },
    ];

    for c in colors {
        println!("{}", c);
    }
}
