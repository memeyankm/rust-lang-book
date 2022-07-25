fn main() {
    another_fn(69);

    let x = five();
    println!("The value of x is: {x}");

    expression_fn();
}

fn another_fn(x: i32) {
    println!("The value of x is: {}", x);
}

fn expression_fn() {
    let y = {
        let x = 3; // This block calls a new macro
        x + 1
    };

    println!("The value of y is: {y}");
}

fn five() -> i32 {
    5
}
