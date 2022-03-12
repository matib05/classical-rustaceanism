fn main() {

    //---------------------------------------------------------------------------------------------------------
    //mut and let (let defaults to immutable)
    let mut x = 6; //notice mut keyword to make this variable mutable. variables are immutable by defualt
    println!("The value of x is {}", x);
    x = 5;
    println!("The value of x is {}", x);

    //---------------------------------------------------------------------------------------------------------
    //Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{}", THREE_HOURS_IN_SECONDS);

    //---------------------------------------------------------------------------------------------------------
    //shadowing
    let x = 1;

    let x = x + 2; 

    {
        let x = x * 2;
        println!("The value of x in inner scope is {}", x);
    }

    println!("The value of x in outer scope is {}", x);

    //Another example of shadowing:
    let spaces = "    ";
    let _spaces = spaces.len(); //Shadowing thus spares us from having to come up with different names
    //Cannot do the following:
    // let mut spaces = "    ";
    // spaces = spaces.len()  <-- compiler error


    //compound type:
    // 1. Tuple: fixed length, and UNFIXED type
    let tup: (i32, f64, u8) = (30, 5.6, 3);
    let (_x,y, z) = tup; // you can destructure and then access the element in the tuple
    println!("The value of y is {}", y);
    println!("The value of x is {}", tup.0); // or you can simply use dot notation to access the respective elemnt
    println!("The value of z is {}", z); // for cmpletion sake

    //2. Array type: fixed length, FIXED single type:
    let array = [2, 4, 3, 600, 4]; // the type is interpreted
    let array: [u32; 5] = [2, 4, 3, 600, 4]; // the type is explicit
    let array = [3; 5]; // this means [3, 3, 3, 3, 3];
    println!("the third element in the array is: {}", array[2]); //you don't use dot notation for arrays
    run_program()

    // Also, I was trying to figure out if the target directory should also be committed, but fell across this:
    // https://medium.com/codex/rust-modules-and-project-structure-832404a33e2e
    
}


use std::io;

fn run_program() {
    let a = [1,2,3,4,5];

    let mut index = String::new();

    println!("Please enter an index:");
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let number = a[index];

    println!("The value of the element at index {} is {}",
        index, number
    );
}