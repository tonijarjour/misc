pub fn flow() {
    let n = 6;

    // if else branches must all return the same type
    let t = if n < 6 {
        println!("{} is less than 6!", n);
        1
    } else if n % 2 == 0 {
        println!("{} is greater than or equal to 6 and even.", n);
        2
    } else {
        println!("What?");
        0
    };

    println!("{}", t);

    // infinite loop
    let mut count = 0;
    loop {
        count += 1;

        if count == 7 {
            // rest of the loop block will be skipped on continue
            continue;
        }

        if count == 7 {
            println!("{}", count);
            break;
        }

        if count == 9 {
            println!("{}", count);
            break;
        }
    }

    // nested loop controls
    let val = 'outer: loop {
        print!("Out ");
        'inner: loop {
            print!("In ");

            // return from loop with break;
            break 'outer 5;
        }
        // unreachable
        println!("Out");
    };

    println!("Done {}", val);

    let mut n = 1;
    while n <= 15 {
        if n % 3 == 0 {
            print!("fizz");
        }
        if n % 5 == 0 {
            print!("buzz");
        }
        if n % 3 != 0 && n % 5 != 0 {
            print!("{}", n);
        }
        println!();
        n += 1;
    }

    for n in 1..=12 {
        print!("{n} ");
    }

    let mut names: Vec<String> = vec!["Alice", "Jane", "Laura"]
        .into_iter()
        .map(String::from)
        .collect();

    // immutable borrow
    for n in names.iter() {
        println!("{n}"); // n is &String
    }

    // mutable borrow to change values
    for n in names.iter_mut() {
        *n = format!("Hello {n}"); // n is &mut String
    }


    // move
    for mut n in names {
        // names.into_iter() moves the vec
        n = format!("{n}, how are you?"); // n is mut String
        println!("{n}");
    }

    // Cannot use names since it was moved
    //println!("{}", names[0]);

    // match
    let num = 1;

    let v = match num {
        1 => {
            println!("One");
            0
        }
        _ if (num % 2 == 0) => {
            println!("{num} is even");
            1
        }
        _ => {
            println!("{num} is odd");
            2
        }
    };

    println!("{v}");

    // destructuring

    let triple = [0, 2, 4, 3, 5];
    match triple {
        [0, start @ .., 3, x] => println!("0 then {start:?} then 3 second to last then {x}"),
        [x, .., y] => println!("{x}{y}"),
        _ => println!("the end"),
    }

    enum Color {
        Red,
        Blue,
        Green,
        White,
        Black,
        RGB(u8, u8, u8),
    }

    impl Color {
        fn as_hex(&self) -> String {
            match self {
                Self::Red => String::from("#FF0000"),
                Self::Blue => String::from("#0000FF"),
                Self::Green => String::from("#00FF00"),
                Self::Black => String::from("#000000"),
                Self::White => String::from("#FFFFFF"),
                Self::RGB(r, g, b) => {
                    format!("#{:X}{:X}{:X}", r, g, b)
                }
            }
        }
    }

    let color = Color::RGB(25, 30, 180);
    println!("{}", color.as_hex());
    println!("{}", Color::Black.as_hex());
    println!("{}", Color::White.as_hex());

    let refer = &3;

    match refer {
        &v => println!("{v}"),
    }
    match *refer {
        v => println!("{v}"),
    }

    let ref refer = 4;
    println!("{refer}");

    let v = 5;
    match v {
        ref f => println!("{f}"),
    }

    struct Stuff<'a> {
        a: u8,
        b: &'a str
    }

    let thing = Stuff { a: 2, b: "Pies" };

    match thing {
        Stuff { a, b: "Tacos" } => println!("You are eating {a} tacos!"),
        // match guard
        Stuff { a, b } if a >= 3 => println!("lots of {b}"),
        Stuff { a, b } if a < 3 => println!("few {b}"),
        // compiler doesnt check guard conditions so this is required 
        Stuff { a, b } => unreachable!("unreachable")
    }

    // binding with Option destructuring
    fn getAge() -> Option<u8> {
        Some(20)
    }

    match getAge() {
        Some(0) => println!("unborn"),
        Some(n @ 1..=12) => println!("preteen"),
        Some(n @ 13..=19) => println!("teen"),
        Some(n) => println!("old"),
        None => println!("non existant")
    }

    // if let as an alternative to match
    let mut opt = Some(5);
    let non: Option<u8> = None;

    if let Some(i) = non {
        println!("{i}");
    } else if false {
        println!("worked");
    } else {
        println!("complete fail");
    }

    // while let alternative to loop > match
    while let Some(i) = opt {
        println!("{i}");
        if (i > 0) {
            opt = Some(i-1);
        } else if (i == 0){
            opt = None;
        }
    }
}
