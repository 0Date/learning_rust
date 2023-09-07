use std::io;

#[allow(dead_code)]

pub fn arithmatic_and_type_casting_main() {
    let x1: u8 =   1;  // range:    0 to 255
    let y1: u8 = 255;  // range: -128 to 127

    //x1 + y1 will not work as the max value up u8 is 255 and the newvariable will inherit the data type therfuore causing an overflow error
    //any arithmetic operation that is set to equal a new variable will have the dat type set to the same as the components of the data type
    
    /*
    Different math operators
    '+' -> addition
    '-' -> subtraction
    '*' -> multiplication
    '/' -> division
    '%' -> remainder
    */

    let x2 = 127_000i64;
    let y2 = 127_000 as i32;

    //both are valid for explicitly strating the dat type of the variable
    //the as operator allows for explicit type conversion

    let z2 = x2 / (y2 as i64);
    println!("Output of the equation: {}", z2);

    //parentheses ar not needed for explicit type conversion however it looks good

    let mut input = String::new();                                           //convered in input_from_console.rs
    io::stdin().read_line(&mut input).expect("failed to read line");        //convered in input_from_console.rs
    
    let int_input: i32 = input.trim().parse().unwrap();
    println!("{}", int_input + 1);

    /*
    modifies input to int_input in 3 steps
      1 - trims "whitespace" characters from input (such as the enter key - can be customized to any char you want)
      2 - parse checks that the previous function succeeded and creates an option value internally that then should be handled
      3 - unwrap causes a panic if the function's option value is of type Err or otherwise marked as unrecoverable (any error type will cause the program to unwrap and shutdown) 
    */

    println!("Ignore------------------------------------------");
    println!("{}", x1);
    println!("{}", y1);
    println!("Ignore end--------------------------------------");
}