use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // Makes a number
    let secret_number: u32 = rand::thread_rng().gen_range(1..100);
    // println!("The secret number is: {secret_number}");
    
    println!("Guess the number!");
    println!("You have 5 chancees; Good luck.");
    println!("-------------------------");

    // This loop is for counting the number of guesses
    let mut count = 5;
    while count > 0 {

        // We take a number from the user and put in a variable
        println!("Please enter your guess.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        // We convert user input from String to int(u32)
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        // We check whether the user`s guess is equal to the secret number or not
        if checker(&guess, &secret_number) {
            // if it was eqaul, get out
            break;
        }

        // We count the number of remaining guesses
        count -= 1;
        if count == 0 {
            println!("Game Over");
        } else {
            println!("Your chances: {count}");
            println!("-*-*-*-*-*-*-*-*-*-*-*-");
        }
    } 
}

fn checker(input: &u32, secret_number: &u32) -> bool{   
    match input.cmp(secret_number) {
        Ordering::Less => { println!("Too small!"); false }
        Ordering::Greater => { println!("Too big!"); false }
        Ordering::Equal => { println!("You win!"); true }
    }
}