pub fn variable_bindings() {
    let int: u8 = 1;

    // _ to silence unused warnings
    let _bol: bool = true;
    let _unit: () = ();

    // copy an integer
    let _copy_int = int;

    // mutable variable
    let mut val = 1;
    val += 1;
    println!("{}", val);

    {
        // value is only 3 inside this block
        let val = 3;
        println!("{}", val);

        let _died = 4;
    }
    
    println!("{}", val);

    // died no longer exists
    //println!("{}", _died);

    // shadowing over the previous val
    let val = 5;
    println!("{}", val);

    let variable;
    let mut changeable = 4;

    {
        // initialization and mutation live past the scope
        variable = 3;
        changeable = 5;
    }

    println!("{} {}", variable, changeable);

    let mut frozen = 2;

    {
        let frozen = frozen;

        // cannot assign immutable variable
        //frozen = 9;
    }

    frozen = 8;
}
