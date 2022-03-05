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
    let spaces = spaces.len(); //Shadowing thus spares us from having to come up with different names
    //Cannot do the following:
    // let mut spaces = "    ";
    // spaces = spaces.len()  <-- compiler error


}
