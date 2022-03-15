fn main() {
    let number = 5;

    if number ==  5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number_two = if number == 5 { 5 } else { 10 }; // i.e. if statements are expressions, so this is valid
    println!("the number is {}", number_two);

    // let number_three = if number == 5 { 5 } else { "ten" }; <-- compile error because of return types of both do not match

    // _______________________________________________________________________________________________________________________
    //LOOPS
    let mut count = 0;
    'counting_up: loop { // the 'counting_up is called a label in rust
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if (count == 2) { //the parenthesis around the if will generate a warning
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    };

    //Returning Values from Loops
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter + 2;
        }
    };
    println!("The result is {}", result);


    // while loops:
    let mut number = 3;
    while number != 0 {
        println!("{}", number);
        number -= 1;
    }

    // for loop
    let a: [i32; 5] = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }

    // for loop using range library:
    for element in (1..4).rev() { // .rev() is reverse and the (1..4) creates a range;
        println!("{}!", element);
    }
    println!("LIFTOFF!!!");
}
