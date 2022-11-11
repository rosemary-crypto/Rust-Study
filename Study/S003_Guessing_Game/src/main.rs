/*************
The program will generate a random integer between 1 and 100. It will then prompt the player to enter a guess. After a guess is entered, the program will indicate whether the guess is too low or too high. If the guess is correct, the game will print a congratulatory message and exit.
**************/

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
	let secret_number = rand::thread_rng().gen_range(1..=100); //rust doesnt yet include random number functionality in its standard library, however the Rust team does provide a rand crate
	//println!("The secret number is: {secret_number}");
	loop{
		println!("Please input your guess."); //typing quit will quit the game
		
		let mut guess = String::new();
		
		io::stdin()
			.read_line(&mut guess)
			.expect("Failed to read line");
			
		//let guess: u32 = guess.trim().parse(){.expect("Please type a number!");
		let guess: u32 = match guess.trim().parse(){
			// Rust allows us to shadow the previous value of guess with a new one. trim() eliminates any whitespace (and \n or \r\n) at the beginning and end. The colon (:) tells rust we'll annotate the variable's type
			Ok(num) => num,
			Err(_) => continue,
		};
		
		println!("You guessed: {guess}"); //or println!("You guessed: {}",guess)
	
		match guess.cmp(&secret_number){ 
		//a match expression is made up of arms. An arm consists of a pattern to match agains, and the code it should run if the value given to match fits that arm's pattern.
			Ordering::Less => println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal => {
				println!("You win!");
				break;
			}		
		}
	}	
}
