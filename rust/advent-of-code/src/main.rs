mod one;
mod three;
mod two;

fn main() {
    println!("\nDay One:");
    println!("{}\n{}", one::solve(1), one::solve(3));

    println!("\nDay Two:");
    println!("{:?}", two::solve());

    println!("\nDay Three:");
    println!("{}", three::solve());
}
