#[allow(dead_code)]

pub fn functions_expressions_and_statements_main() {
    demo_one();
    demo_2(2.5, 9.5);
    //to set local variable values pass the value wanted through the function call in the order of the variables
    let result = demo_3(1, 2);
    println!("result is: {}", result);
}

fn demo_one() {
    //to create a function use the fn keyword
    //when naming a function use snake case for ease of readability
    println!("demo has been called")
}
//functions could be created before the main function in a normal file and would be ignored until the main function calls it whether directly or indirectly

fn demo_2(x: f32, y: f32) {
    let z = x + y;
    println!("Sum is: {}", z);
    //local variables for functions need to be declared withing the partheses 
    //x and y are local variables created within the demo_2 function
    //type is required to be defined alongside the variables
}

//a statement is something like a variable declaration and doesnt return a value
//Example: let x = 20;

//an expression is something that returns a value
//Example println!("Hello world!");
//because macros like 'println!()' return a value they are viewed as expressions

/*
Another example of an expression is:
let number = {
    let x = 3;
    x + 1
};

The reason this is an expression is because it returns the value x + 1
the reason 'x + 1' doesnt have a semicolon after it is due to the fact that 'x + 1' is a function because it returns the value (x +1) back to number
*/

fn demo_3(x: i32, y: i32) -> i32 {
    x + y   //dont add a semicolon in order to make this an expression not a statement 
}
//the '-> opperator allows you to specify the returned value type
// the 'return' keyword can also be used infront of the point that you wish to express which would then allow you to use a semicolon at the end of the expression 