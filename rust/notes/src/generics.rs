use std::fmt::Display;

pub fn generics() {
    // function that takes generic param of type T
    fn _foo<T>(_a: T) {}

    // generic struct
    struct A;
    struct ContainsA(A);
    struct GenStruct<T>(T);

    let _char_struct: GenStruct<char> = GenStruct('a');
    let _a_struct = GenStruct(A);
    let _a_contains = GenStruct(ContainsA(A));

    // not generic
    fn take_a(_a: A) {}

    // not generic, a can only be a genstruct with A inside
    fn take_gen_a(_a: GenStruct<A>) {}

    // take a GenStruct with a generic T
    fn gen_take_gen<T>(_t: GenStruct<T>) {}

    take_a(A);
    //take_gen_a(GenStruct(5));
    take_gen_a(GenStruct(A));
    gen_take_gen(GenStruct(A));
    gen_take_gen(GenStruct('a'));
    // function call that explicitly takes u8
    gen_take_gen::<u8>(GenStruct(5));

    // impl for GenStruct over u8
    impl GenStruct<u8> {
        fn add_one(&self) -> Self {
            Self(self.0 + 1)
        }
    }

    // impl for generic GenStruct
    impl<T> GenStruct<T> {
        fn val(&self) -> &T {
            &self.0
        }
    }

    let five = GenStruct::<u8>(5);
    let six = five.add_one();
    println!("{} {}", five.val(), six.val());

    // generic trait
    trait DoubleDrop<T> {
        fn double_drop(self, _: T);
    }

    // implement trait for generic caller and parameter
    impl<T, U> DoubleDrop<T> for U {
        fn double_drop(self, _: T) {}
    }

    let bs = A;
    let cs = A;

    cs.double_drop(bs);
    //bs; cs;

    // type bound
    fn greet<T: Display>(t: T) {
        println!("Hello, {t}");
    }
    struct S<T: Display>(T);
    // Error because Display is not implemented for Vec
    //let s = S(vec![1]);

    // implement Display for S with param T that implements Display
    impl<T: Display> Display for S<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    // Now printing the struct is possible
    println!("{}", S("Bob"));

    // Empty bound
    trait Printable {}
    impl<T: Display> Printable for S<T> {}

    // multiple bounds
    fn is_printable<T: Display + Printable>(_: &T) -> bool {
        true
    }

    // where clause
    struct Thing<T, U>
    where
        T: Display,
        U: Display,
    {
        t: T,
        u: U,
    }

    // associated types
    // avoids having to specify the types again if already contained

    /*
    trait Special<Unique, Useful> {
        fn is_special(&self, _: &Unique, _: &Useful) -> bool;
    }
    fn do_if_special<Unique, Useful, T>(t: &T)
    where T: Special<Unique, Useful> {}
    */

    trait Special {
        type Unique;
        type Useful;

        fn is_special(&self, _: &Self::Unique, _: &Self::Useful) -> bool;
    }

    fn do_if_special<T: Special>(t: &T) {}
}
