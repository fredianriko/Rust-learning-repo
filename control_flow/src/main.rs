fn main() {
    // println!("Hello, world!");

    // IF expression

    // let number = 3;

    // if number < 5 {
    //     println!("Condition was trues");
    // } else {
    //     println!("Condition was false");
    // }

    // if in let statement

    // let condition = true;
    // let number = if condition { 5 } else { 6 };
    // println!("The value of number is : {number}");

    // Repetition wit LOOPS
    // infinitely looping until explicitly tells to stop
    // loop {
    //     println!("again!");
    // }

    // let mut counter = 0;
    // let result = loop {
    //     counter += 1;

    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };
    // println!("Counter value is : {result}");

    // loops labels

    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("count = {count}");
    //     let mut remaining = 10;

    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }

    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }
    //     count += 1;
    // }
    // println!("End count = {count}");

    // Conditional loop with while
    // let mut number = 3;
    // while number != 0 {
    //     println!("{number}");
    //     number -= 1;
    // }

    // println!("LIFTOFF!!!");

    // looping through collection
    // let a = [10, 20, 30, 40, 50];
    // let mut index = 0;

    // while index < 5 {
    //     println!("The value is {}", a[index]);

    //     index += 1;
    // }

    // more concise looping for each

    // let a = [10, 20, 30, 40, 50];

    // for element in a {
    //     println!("{element}");
    // }

    // for in range - reversed (rev)

    let _a = [10, 20, 30, 40, 50];

    for element in (1..4).rev() {
        println!("{element}");
    }
}
