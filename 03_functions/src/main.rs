fn main() {
    println!("Hello, world!");
    another_function(5);
    print_labeled_measurement(5, 'h');
    statements_only('x'); //This is an expression because it returns something;
    expressions_only(); // this is also an expression. All functions are expressions;
    println!("The value of the function that returns a value is: {}", function_that_returns_a_value());
}

fn another_function(x: i32) {
    // In function signatures, you must declare the type of each parameter. This is a deliberate decision in Rust’s design: 
    // requiring type annotations in function definitions means the compiler almost never needs you to use them elsewhere 
    // in the code to figure out what type you mean.
    println!("the value of x is {}", x);
}

fn print_labeled_measurement(measurement_value: i32, measurement_type: char) {
    println!("The measurement provided is {}{}",
        measurement_value, measurement_type
    );
}

fn statements_only(x: char) {
    let _a: char = x; //bad practice to include underscore just to get the linter to stop complaining.c
    // this is a statement and the resulting function is only a statement because it does not return anything, it just does something.
    // actually the entire function definition is a statement;
    // SEMICOLON MEANS IT IS A STATEMENT;
}

fn expressions_only() {
    let y = { // this is a expression because all scopes opening with brakcets are expressions
        let x = 5;
        x + 2 // this is a statement because it does not have a semicolon. It is returning a value. expressoins return value.
        // If i had added a semicolon here, it would make it into a statement, and no longer return a value;
    };

    println!("the value of y is: {}", y);
}

fn function_that_returns_a_value() -> i32 {
    3 // Two points:
    // 1. if I were to put a semicolon here, it would fail, because that would mean I am no longer returning a type, instead
    //      I converted the intended expression to a statement;
    // 2. The errors in rust are always instructive and not just expository;
}