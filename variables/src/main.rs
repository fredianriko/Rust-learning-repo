fn main() {
    // MUTABLE
    // "let" keyword is a keyword to create a variabel in rust, a variabel created using let is immutable by default
    // to make it mutable, you need to add "mut" keyword after it

    // let mut x = 5;
    // println!("The value of x is {}", x);
    // x = 10;
    // println!("The value of x is {}", x);

    // CONSTANT
    // constants are values that are bound to a name and are not allowed to change
    // Constants can be declared in any scope, including the global scope,
    // which makes them useful for values that many parts of code need to know about.
    //Rust’s naming convention for constants is to use all uppercase with underscores between words.

    // const THREE_HOUR_IN_SECONDS: u32 = 60 * 60 * 3;
    // println!("The value of constant {THREE_HOUR_IN_SECONDS}");

    // SHADOWING
    // shadowing is a method

    // let x = 5;
    // println!("The value of x is {x}");

    // let x = 10; // redeclare the variabel x , this is called "shadowing" the same variabel previous value
    // println!("The value of x is {x}");

    // DATATYPES

    // let gusss: u32 = "42".trim().parse().expect("Not a number");
    // println!("Guess is {gusss}");

    // SCALAR TYPE

    // integers: unsigned and signed (non-negative number and possible negative number)
    // floating-point :  f32 and f64 (single precision float point and double precision float point)
    // Booleans : bool (true false)
    // characters : single character 'A' => must use single quote

    // COMPOUND TYPE :
    // can group multiple values into one type (tuples and arrays)

    // Tuple type
    // general way to grouping a number with other type into one compound type
    // you can destructure value from tuple in to a variabel name like the below example

    // let tup: (i32, f64, u8) = (500, 6.4, 1);

    // let (x, _y, _z) = tup;
    // println!("Value x destructured from tuple {x}");

    // tuple can be accessed directly using dot (.)
    // let x: (i32, f64, u8) = (500, 64, 1);
    // let five_hundred = x.0;
    // let six_point_four = x.1;
    // let one = x.2;

    // tuple without any value valled "unit"
    // let units: () = (); // this is called unit

    // Array type
    // Unlike a tuple, every element of an array must have the same type.

    // let a = [1, 2, 3, 4, 5];
    // let months = [
    //     "January",
    //     "February",
    //     "March",
    //     "April",
    //     "May",
    //     "June",
    //     "July",
    //     "August",
    //     "September",
    //     "October",
    //     "November",
    //     "December",
    // ];

    // defining array by its type and the length of it
    // here you defined an array that the elements of it has type of signed 32 bit integer
    // and have 5 element in it
    // let a: [i32; 5] = [1, 2, 3, 4, 5];

    // defining array that has the same value for all element in it
    // let a = [3; 5]; // [3,3,3,3,3]

    // accessing array through index
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("first value {first}");
    println!("second value {second}");

    // the compiler will panicked out of bound, when you are trying to access element that is outside of the array size
}

// NOTES

// "mut" keyword cannot be used with constant
// constant always immutable
