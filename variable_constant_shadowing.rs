#[allow(dead_code)]

pub fn variable_constant_shadowing_main(){
    let x = 4;          //implicit type because x's type is defined by compiler
    println!("x is {}", x);  //imbed variable into string

    //cant change value of x because rust doesnt automatically allow mutability to variables

    let mut y = 1;
    println!("y is {}", y);
    y = 3;
    println!("y is {}", y);

    //To make a variable mutable you have to use the mut keyword after the keyword let

    let z = 10;
    println!("z is {}", z);
    let z = 5;
    println!("z is {}", z);

    //Variables can be changed without being mutable without using mut keyword however the let keyword will need to be used

    let z = z + 1;
    println!("z is {}", z);

    //variables can be incremented by adding or multiplying a number onto the variable using the let keyword

    {
        let z = 4;
        println!("z is {}", z);
    }

    //variables inside of scopes (curly brackets) can have the same name as those outside the scopes and not interfer with one another
    //You can also incriment variables inside of a scope without afecting those outside the scope

    let z = "string";
    println!("z is a {}", z);

    //when redifining a variable you can change the type however if the variable is mutable then the type is inmutable

    const CONSTANT_NAME: u32 = 60;
    println!("constant is {}", CONSTANT_NAME);

    //cantants are imutable and cant not have their type changed even if the constant is redeclared
    //The name of the constant should be in capital snake case 
    //The type (int,string,float) is required to be declared and cannot be implicit (chosen by compiler)
}