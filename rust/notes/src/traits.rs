pub fn traits() {
    trait Animal {
        fn name(&self) -> &'static str;
        fn sound(&self) -> &'static str;
    }

    struct Sheep {
        name: &'static str,
    }

    struct Cow {
        name: &'static str,
    }

    impl Animal for Cow {
        fn name(&self) -> &'static str {
            self.name
        }

        fn sound(&self) -> &'static str {
            "moooo"
        }
    }

    impl Animal for Sheep {
        fn name(&self) -> &'static str {
            self.name
        }

        fn sound(&self) -> &'static str {
            "baaah"
        }
    }

    // return an Animal of many
    fn random_animal(rn: u8) -> Box<dyn Animal> {
        if rn < 127 {
            Box::new(Sheep { name: "Monah" })
        } else {
            Box::new(Cow { name: "Betty" })
        }
    }

    println!("{}", random_animal(130).sound());

    // iterator
    struct Fibonacci {
        p: u32,
        c: u32,
    }

    impl Fibonacci {
        fn new() -> Self {
            Self { p: 0, c: 1 }
        }
    }

    impl Iterator for Fibonacci {
        type Item = u32;
        fn next(&mut self) -> Option<Self::Item> {
            let hold = self.p;
            self.p = self.c;
            self.c += hold;

            Some(self.c)
        }
    }

    Fibonacci::new().take(10).for_each(|n| print!("{} ", n));
    println!();

    // super trait

    trait Person {
        fn name(&self) -> &'static str;
    }
    trait Student: Person {
        fn new(name: &'static str) -> Self;
        fn major(&self) -> &'static str;
    }

    struct CompSciStudent {
        name: &'static str,
        major: &'static str
    }

    // must implement person if implementing student
    impl Person for CompSciStudent {
        fn name(&self) -> &'static str {
            self.name
        }
    }

    impl Student for CompSciStudent {
        fn new(name: &'static str) -> Self {
            Self {
                name,
                major: "Computer Science"
            }
        }
        fn major(&self) -> &'static str {
            self.major
        }
    }

    let anton = CompSciStudent::new("Anton");
    println!("{} majors in {}", anton.name(), anton.major());
}
