use std::io;

#[allow(dead_code)]

pub fn input_from_console_main() {
    let mut input = String::new();

    //the "::" operator is the path seperator operator

    io::stdin().read_line(&mut input).expect("failed to read line");

    //this line allows the input from the console to directly modify the 'input' variable
    //the '&mut' makes it so the variable is actually edited because otherwise the variable would get copied and the variable wouldnt get modified

    println!("You typed: {}", input);
}
