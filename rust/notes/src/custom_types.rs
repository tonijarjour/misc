pub fn custom_types() {
    #[derive(Debug)]

    // struct
    struct Person {
        name: String,
        age: u8,
    }

    // tuple struct
    struct RGB(u8, u8, u8);

    // unit struct
    struct Unit;

    #[derive(Debug)]
    struct Point {
        x: f32,
        y: f32,
    }

    impl Point {
        fn origin() -> Self {
            Point { x: 0.0, y: 0.0 }
        }
    }

    // nested structs
    #[derive(Debug)]
    struct Rectangle {
        tl: Point,
        br: Point,
    }

    // new instance
    let jane = Person {
        name: String::from("Jane"),
        age: 23,
    };
    println!("{:?}", jane);

    // access values
    println!("{} is {} years old.", jane.name, jane.age);

    // destructuring into vars x and y
    let br = Point { x: 10.3, y: 0.4 };
    let Point { x, y } = br;

    let tl = Point {
        x: x - 8.9,
        y: y + 13.0,
    };

    // quick input if same name as field
    let rect = Rectangle { tl, br };

    // destruct tuple struct
    let RGB(red, green, blue) = RGB(12, 24, 36);

    impl Rectangle {
        fn rect_area(&self) -> f32 {
            // nested destructuring
            let Self {
                tl: Point { x: lx, y: hy },
                br: Point { x: hx, y: ly },
            } = self;

            (hx - lx) * (hy - ly)
        }

        fn square(p: Point, l: f32) -> Self {
            Self {
                tl: Point { ..p },
                br: Point {
                    x: p.x + l,
                    y: p.y - l,
                },
            }
        }
    }

    println!("{}", rect.rect_area());

    let sqr = Rectangle::square(Point::origin(), 5.0);
    println!("{} tl: {:?} br: {:?}", sqr.rect_area(), sqr.tl, sqr.br);

    // enums can hold values
    enum Happening {
        Start,
        First,
        Past(u32),
        Current(bool),
        Future(String),
        Last,
        End { n: String, t: u32 },
    }

    impl Happening {
        fn resolve(&self) {
            match self {
                Self::Start => println!("The beginning"),
                Self::First => println!("Number One"),
                Self::Past(x) => println!("Was at {} seconds", x),
                Self::Current(b) => println!("Happening? {}", b),
                Self::Future(e) => println!("The {} will come", e),
                Self::Last => println!("The last is not the end."),
                Self::End { n, t } => println!("The end. {} {}", n, t),
            }
        }
    }

    Happening::Start.resolve();
    Happening::First.resolve();
    Happening::Past(5).resolve();
    Happening::Current(true).resolve();
    Happening::Future(String::from("end")).resolve();
    Happening::Last.resolve();
    Happening::End {
        n: String::from("Exit"),
        t: 40,
    }
    .resolve();

    println!();

    // type alias
    type Happ = Happening;

    // Use brings enum values into scope
    use Happening::{First, Last, Start};

    Start.resolve();
    First.resolve();
    Last.resolve();
    Happ::Current(false).resolve();

    println!();

    // C like enum
    enum Color {
        Zero,
        Red = 0xff0000,
        Green = 0x00ff00,
        Blue = 0x0000ff,
    }

    println!("Zero {}", Color::Zero as u32);
    println!("Red {}", Color::Red as u32);

    // linked list with enums
    enum List {
        Element(u32, Box<List>),
        Nil,
    }

    impl List {
        fn new() -> Self {
            Self::Nil
        }

        fn prepend(self, e: u32) -> Self {
            Self::Element(e, Box::new(self))
        }

        fn len(&self) -> u32 {
            match self {
                Self::Element(_, next) => 1 + next.len(),
                Self::Nil => 0
            }
        }

        fn stringify(&self) -> String {
            match self {
                Self::Element(v, next) => {
                    format!("{}, {}", v, next.stringify())
                },
                Self::Nil => format!("Nil")
            }
        }
    }

    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("List Length: {}", list.len());
    println!("List: {}", list.stringify());


    // const and static
    // Accessing or modifying a mutable static is unsafe
    const LANGUAGE: &str = "Rust";
    static AUTHOR: &str = "Toni Jarjour";

    println!("Written in {} by {}", LANGUAGE, AUTHOR);
}
