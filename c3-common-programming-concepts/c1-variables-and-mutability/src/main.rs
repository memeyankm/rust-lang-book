// variables are by default immutable

fn main() {
    // VARIABLES: Immutable, unless mut is used
    let x = 5;
    println!("The value of x is: {}", x);

    // x = 6;
    // println!("The value of x is: {}", x);
    // Doesn't work as x is immutable

    let mut y = 5;
    println!("The value of y is: {}", y);

    y = 6;
    println!("The value of y is: {}", y);
    // Works since y is mutable

    // CONSTANTS: Always Immutable
    const 1_DAY_IN_SECONDS: u32 = 60 * 60 * 24;
    println!("One day in seconds: {}", 1_DAY_IN_SECONDS);

    // SHADOWING: When a variable is declared with the same name as a previous variable, 
    // the previous variable is shadowed.

    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
    // x is now 10

    let spaces = "   ";
    let spaces = spaces.len();
    // The second spaces is int32 while first is str
    // in case of mut dtype is fixed to str
}