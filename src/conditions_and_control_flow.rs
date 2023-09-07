#[allow(dead_code)]

pub fn conditions_and_control_flow_main() {
    /*
    Different opperators
    <       less than
    >       greater tham 
    <=      less than or equal to
    >=      greather than or equal to
    !=      is not equal to
    ==      is equal to

    When using these values the operation being done will return a true or false
    */

    let cond = 2 < 3;
    println!("{}", cond);

    //can not compare objects of different types 
    //Example: (2 <= 2.2) is true however rust isnt able to compare differentiating types

    /*
    Logical operators in rust are
    &&      and
    ||      or
    !       not
    */

    let cond2 = false;

    let cond3 = cond && cond2;      //returns true if both inputs are true
    let cond4 = cond || cond2;      //returns true if cond or cond 2 is true
    let cond5 = !(cond || cond2);   //returns the oppasite od the value in the paretheses

    /*
    the order in which logical opperators are applied
    ! is applied first
    && is applied second
    || is is applied last

    the exception is when there are partheses in which case the parentheses are applied first
    */
    println!("cond3: {}", cond3);
    println!("cond4: {}", cond4);
    println!("cond5: {}", cond5);

    println!("beginning of control flow");

    let food = "cookie";

    if food == "cookie" {
        println!("yum");
    } else {
        println!("loser doesnt have cookie")
    }

    //if statement checks the 2 inputs given against eachother using the logical operator provided
    //if the arguments check out with the logical operators then the if statements will run
    //however if the if statement and logical operators dont check out then the if statement doesnt run however if an else statement is provided that will run

    if food == "cookie" {
        println!("yum");
    } else if food != "pie" {
        println!("yun")
    } else {
        println!("loser doesnt have pie or cookie")
    }

    //the else if operator works when the if or else if statement beforehand is false
    //the else if statement also requires arguments to be used to check for a boolean and if that returns false then the statement isnt run
    //if the if statement beforehand is true the if stament will break and wont check further
    //if there is an else in the if statement (however long the chain goes) it will always have to come last
}