//  Include neccesary crates
use std::io;
use std::cmp::Ordering;

use rand::Rng;

fn main()
{
    println!("Hello, Rust!");

    // Generate a random number
    let rand_num : u16 = rand::thread_rng().gen_range(1, 101);
    println!("The random number is {}", rand_num);

    loop
    {
        println!("Enter something:");

        let mut guess = String::new();  // Seems inefficient to redeclare string each time

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line"); // Cause a panic in the event of io failure

        let guess : u16 = match guess.trim().parse()
        {
            Ok(num) => num, // Leaving the ; off here leaves num as an expression, hence capable or returning a value to bind to guess
            Err(_) =>
            {
                println!("Please enter a number.");
                continue;
            }
        };

        println!("You entered: {}", guess);
        match guess.cmp(&rand_num)  // This appears similar to switch cases in c/c++
        {
            Ordering::Less =>
            {
                println!("Your number is smaller.");
            },
            Ordering::Greater =>
            {
                println!("Your number is bigger.");
            },
            Ordering::Equal =>
            {
                println!("Your number is the same.");
                break;
            }
        }
    }
}
