// This is a simple number guessing game where the user tries to guess a randomly generated number between 1 and 100. The program generate a number using
// the rand crate and  then prompts number between 1 and 100.. The user input is guess and the program provides feedback on whether the guess is too low,
// too high, or correct. THe game continues until the user guesses the correct number.


// To run the code, we used the library rand, which is popular crate  for generating random numbers in Rust.
// We also used the library std::io for handling user input and output, and std::cmp::Ordering for comparing the user's guess with the secret number.

// In the main function, we start by printing a welcome message to the user. Then we generate a random secret number between 1 and 100 using the rand crate.
// We enter a loop where we prompt the user to input their guess. We read the user's input and attempt to parse it as an unsigned 32-bit integer (u32).
// If the parsing fails (e.g., if the user enters a non-numeric value), We print an error message and continue to the next iteration of the loop,
// allowing the user to try again. If the parsing is succesful we compare the user's guess with the secret number using the cmp method,
// which returns an Ordering (Less, Greater,or Equal). Based on the result of the comparison, we provide feedback to the user.
// If the guess is too low, we print"too small". If the guess is too high, we print "too big". If the guess is correct, we print "you win" and break
// out of the loop,ending the game.




use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main () {
    println!("Guess the number!");



    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("please input your guess");

        let mut guess = String::new();
        
        io::stdin()
             .read_line(&mut guess)
             .expect("failed to read line");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("please type a valid number");
                    continue;
                }
            };
            println!("you guessed: {}", guess);


            match guess.cmp(&secret_number) {
                Ordering::Less => println!("too small"),
                Ordering:: Greater => println!("too big"),
                Ordering::Equal => {println!("you win");

                break;
            }

            }
        }
    }
