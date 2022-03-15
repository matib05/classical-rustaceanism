fn main() {
    let number = 5;

    if number ==  5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number_two = if number == 5 { 5 } else { 10 };
    println!("the number is {}", number_two);

    // let number_three = if number == 5 { 5 } else { "ten" }; <-- compile error because of return types of both do not match

}
