fn main() {
    println!("Hello, world!");
    another_function(5);

    // concatenate
    concate_string(5, 'H');

    // statement
    // adding ";" semicolon to a expression will make the expression into a statement
    // an expression is evaluated -> such as math operation
    // the value of statement is and "expression"
    // let x = 6 -> 6 here is expression, but defining the x is considered as statement
    let y = {
        let x = 10;
        x + 1
    };

    println!("The value of y is {y}");

    // function with return value
    let s = five();
    println!("the value of s is {s}");

    let t = six(5);
    println!("value of t {t}");

    // control flow condition
    let z = condition(10);
    println!("value of condition first {z}");
    let zz = condition_heap(11);
    println!("valud of second condition {zz}");
}

// parameter
fn another_function(x: i32) {
    println!("value of x is {x}");
}

// concatenation

fn concate_string(x: i32, y: char) {
    println!("The measurement is {x}{y}");
}

// function with return value

fn five() -> i32 {
    5
}

fn six(x: i32) -> i32 {
    x + 1
}

// Control flow

fn condition(x: i32) -> &'static str {
    if x < 3 {
        return "Number too small";
    } else {
        return "Number too big";
    }
}

fn condition_heap(x: i32) -> String {
    if x < 3 {
        return "Number too small".to_string();
    } else {
        return "Number too big".to_string();
    }
}
