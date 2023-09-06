#[allow(dead_code)]

pub fn primitive_data_types_main() {
    
    scalar_data_types();
    compound_data_types();

    fn scalar_data_types(){
        //Scalar data types are finite in the possible values they can posses and can be compared with greater than, less than, and/or equal to
        //integers, floating-point, boolean, and character/char are all types of scalar data types

        let i: i32 = 2;

        //i defines the type of the variable
        //32 defines the amount of bits that the integer will use
        //the integer data type can use 8, 16, 32, 64, and 128 bits but rust-analyzer defaults to 32 bits
        //by using i the integer is signed and can use negative numbers

        let u: u32 = 2;
    
        //u means the integer is unsigned and therfor will not accept negtive numbers as th '-' symbol is a sign
        //every other function of 'u' is identicle to 'i'

        let f: f64 = 10.9;

        //f is for floating point numbers the only allowed values for bit numbers is 32 and 64

        let b: bool = false;

        //bool is for the boolean type which stores a true or false statment 
        //'true' can be represented as 1 and 'false' can be represented as 0

        let c: char = 'c';
        //char is for a single character which can be anything
        //char dattype using only single quotes to wrap the character

        //ignore (this is here to stop the compiler from yelling)
        println!("Ignore------------------------------------------");
        println!("i is {}", i);
        println!("u is {}", u);
        println!("f is {}", f);
        println!("b is {}", b);
        println!("c is {}", c);
        println!("Ignore end--------------------------------------");
    }

    fn compound_data_types(){
        //compound data types are made of multiplpe parts of data for example it can have numbers and characters in the same data set
        //tuples and arrays are examples of this data type

        let t: (i32, bool, char) = (1, true, 'l');
        let t2: (i8, bool, char) = (1, true, 'l');
        
        //to define the data type of a tuple you have to define every element in the tuple as tuple itself isnt able to be defined
        //because the data type in t2 uses i8 opposed to i32 in t the types of tuples are different even though each one as the same integer, boolean, and character 
        
        let mut t3: (i32,bool, char) = (1, true, '1');
        println!("{}", t3.0);
        t3.0 = 7;
        println!("{}", t3.0);

        //a tuple is a fixed length of elements that is unmutable by default but can be mutable by using the 'mut' keyword more than one index can be rewritten in a tuple

        println!("{}", t.0);

        //to get the location of the data in the tuple you want start counting from 0 until you get to your index you want
        //the line of code about will print out index 0 which has the value 1
        
        let a: [i32; 5] = [1, 2, 3, 4, 5];

        //arrays are like tuples but have to have the same type inside throughout the entirearray
        //accessing an index in an array is exactly like a tuple except you use square brackets
        //the length of the array is defined by the 5 within the square brackets on the left
        //compiler doesnt automatically initialize array and array has to have some data in it to be initialized

        println!("{}", a[1]);

        //To call a specific part of an array use square brackets around the index you would like to call

        //ignore (this is here to stop the compiler from yelling)
        println!("Ignore------------------------------------------");
        println!("{}", t2.0);
        println!("Ignore end--------------------------------------");
    }
}