pub fn guessing_game() {
    use rand::Rng;
    use std::io::Write;

    let secret = rand::thread_rng().gen_range(1..=100);
    print!("Guess the number!\nPlease input your guess: ");

    std::io::stdout().flush().unwrap();

    loop {
        let mut guess = String::new();

        std::io::stdin().read_line(&mut guess).unwrap();

        let guess: u8 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        match guess.cmp(&secret) {
            std::cmp::Ordering::Less => print!("Lo: "),
            std::cmp::Ordering::Greater => print!("Hi: "),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        std::io::stdout().flush().unwrap();
    }
}
