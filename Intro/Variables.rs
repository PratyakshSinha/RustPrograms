fn main() {
    let mut x = 4;
    println!("x is: {}", x);
    // x = 5 Gives error Variables are immutable in Rust by default so either make them mutable or redefine them
    x = 5;
    println!("x is: {}", x);

    {
        let x = x + 2; // This Works??!!!!
        println!("x is: {}", x);
    }

    let y = 10;
    println!("y is: {}", y);
    let y = y * y; // y is declared again therefore no error
    println!("y is: {}", y);
    let y =
        "The Dark Side of the Force is pathway to many abilities some considered to be unnatural"; // Still works redefinition
    println!("y is: {}", y);

    const VADER_QUOTE: &str = "I find your lack of faith disturbing"; // Constants value and type both cannot be changed in the program once defined
    println!("{}", VADER_QUOTE);
}
