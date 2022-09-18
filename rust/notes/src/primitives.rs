pub fn primitives() {
    // unsinged integers
    // u8, u16, u32, u64, u128, usize
    let a: u8 = 255;
    let b: u32 = 255;
    let c: usize = 255;

    // signed integers
    // i8, i16, i32, i64, i128, isize
    // shadowing, overwite variables
    let a: i8 = -120;
    let b: i32 = -255;
    let c: isize = -255;

    // floating point
    let a: f32 = -1.3;
    let b: f64 = 3.1415;

    // char, bool, unit
    let a: bool = true;
    let mut b: char = 'b';
    let c: () = ();

    // mutable variables
    b = '$';

    // underscores in long numbers
    let c = 1_000_000;

    println!("{} {}{}", a, b, c);

    // operations
    println!("{}", 1u8 + 4);
    println!("{}", 1i8 - 4);

    println!("{}", true && false);
    println!("{}", true || false);
    println!("{}", !false);

    // bitwise opertions
    println!("1010 & 0010 = {:04b}", 0b1010 & 0b0010);
    println!("1010 | 0010 = {:04b}", 0b1010 | 0b0010);
    println!("1010 ^ 0010 = {:04b}", 0b1010 ^ 0b0010);

    // shifts
    println!("1 << 6 = {}", 1 << 6);
    println!("128 >> 6 = {}", 128 >> 6);

    // tuples and destructuring
    let nc: (u8, char) = (5, 'c');
    let (n, c): (u8, char) = nc;

    println!();

    println!("{n} {c}");
    println!("{} {}", nc.0, nc.1);

    println!();

    // nested tuples
    let ab: ((u8, u8), (u8, u8)) = ((8, 9), (6, 7));
    println!("{} {} {} {}", ab.1 .0, ab.1 .1, ab.0 .0, ab.0 .1);

    println!("{:?}", ab);

    struct Matrix(f32, f32, f32, f32);

    println!();

    impl std::fmt::Display for Matrix {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "( {} {} )\n( {} {} )", &self.0, &self.1, &self.2, &self.3)
        }
    }

    let mtx = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix\n{}", mtx);

    impl Matrix {
        fn transpose(&self) -> Self {
            Self(self.0, self.2, self.1, self.3)
        }
    }

    println!();

    println!("Transpose\n{}", mtx.transpose());

    println!();

    // arrays and slices
    let arr: [u8; 5] = [1, 2, 3, 4, 5];
    let frr: [usize; 255] = [1; 255];

    println!("{}", arr[0]);
    println!("{} {}", frr[120], frr.len());

    // arrays are stack allocated
    // slices point to part of an array
    // end index is non inclusive unless [start .. =end]
    println!("{}", std::mem::size_of_val(&frr[90 .. 230]));

    // access array indices safely with .get(i)
    for i in 240..260 {
        match frr.get(i) {
            Some(v) => print!("{} ", v * i),
            None => print!("\n{i} too far")
        }
    }

    println!();

    // indexing out of bounds causes a runtime error
    println!("{}", frr[255]);
}
