// no restriction on order of functions
pub fn functions() {
    println!("{}", hello());
    say_hello();

    let p = Point { x: 5, y: 3 };
    println!("{p}");
    let mut p = p.translate(3, 1);
    println!("{p}");
    p.translate_mut(2, 2);
    println!("{p}");
    let o = Point::new(3, 3);
    println!("{o}");
    o.delta(p);

    // closures
    let eveninc = |i: u8| -> u8 {
        if i % 2 == 0 {
            i + 1
        } else {
            i
        }
    };
    let closure = |i| i + 1;
    println!("{}", closure(5)); // 6
    println!("{}", eveninc(8)); // 9

    let mut count = 1;
    // mut required as calling the closure mutates it
    let mut closure = || count += 1;
    closure();

    // force move of captured variable with move ||
    let name = String::from("Jane");
    let movename = move || println!("{name}");
    movename();
    // name was moved into movename
    //println!("{}", name);

    let sub = |a: i8, b: i8| {
        println!("{}", a - b);
    };

    let add = |a: i8, b: i8| {
        println!("{}", a + b);
    };

    // Fn only accepts a reference
    fn apply<F: Fn(i8, i8)>(f: F, a: i8, b: i8) {
        f(a, b);
    }

    apply(add, 5, 3);
    apply(sub, 9, -12);

    // FnMut can also accept a mutable reference
    fn alter<F: FnMut()>(mut f: F) {
        f();
    }

    let mut person = String::from("Toni");
    let hello_to = || person = format!("Hello, {person}");

    alter(hello_to);

    // FnOnce can also accept a move
    fn destroy<F: FnOnce()>(f: F) {
        f();
    }

    let str = String::from("Hello");
    let drop_str = || std::mem::drop(str);

    destroy(drop_str);

    // functions can also be passed
    fn say_hi() {
        println!("Hello!");
    }
    fn call<F: Fn()>(f: F) {
        f();
    }
    call(say_hi);

    // returning closures
    fn create_fn(s: String) -> impl Fn() {
        // move indicates captures occur by value not reference
        move || println!("Hello, {}!", s)
    }

    let subject = String::from("World");
    // subject moved into create_fn
    let test = create_fn(subject);
    test();
    // cannot use a moved value
    //println!("{}", subject);

    let v = vec![1, 2, 3, 4];
    // pass a closure returning bool that applies a check to each vec element
    let has_2 = v.iter().any(|&x| x == 2);
    println!("{has_2} {v:?}");

    match v.iter().position(|&x| x == 3) {
        Some(n) => println!("Found 3 at {n} {v:?}"),
        None => println!("Not Found"),
    };

    match v.iter().find(|&&x| x == 3) {
        Some(n) => println!("Found {n} {v:?}"),
        None => println!("Not Found"),
    };

    // find_map applies the closure to each element and returns the first Some()
    match v
        .iter()
        .find_map(|&x| if x % 2 == 0 { Some(x * 2) } else { None })
    {
        Some(n) => println!("{n}"),
        None => println!("No Even Values"),
    };

    // Higher Order Functions
    let sum_odd_squares: u32 = (0..).map(|n| n*n) // square all
        .take_while(|&n| n < 5000) // take square results up to 5000
        .filter(|&n| n % 2 == 1) // that are odd
        .sum(); // sum them

    println!("{sum_odd_squares}");

}

fn hello() -> String {
    String::from("Hello Functions!")
}

fn say_hello() {
    // -> ()
    println!("Hello, Functions!");
}

struct Point {
    x: u8,
    y: u8,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "• {} {}", self.x, self.y)
    }
}

impl Point {
    // associated functions
    fn origin() -> Self {
        Self { x: 0, y: 0 }
    }

    fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }

    // method
    fn delta(&self, n: Point) {
        println!("Δ {} {}", n.x - self.x, n.y - self.y);
    }

    // move method, original is destroyed
    fn translate(self, x: u8, y: u8) -> Self {
        Self {
            x: self.x + x,
            y: self.y + y,
        }
    }

    // mut method requires mut caller
    fn translate_mut(&mut self, x: u8, y: u8) {
        self.x += x;
        self.y += y;
    }
}
