use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    check_number();
}

fn check_number() {
    let rng = rand::thread_rng().gen_range(1..=20);
    
    loop {
	println!("Input your guess, please: ");

	let mut guess = String::new();

	io::stdin().read_line(&mut guess).expect("Failed to read line.");

	let guess: u32 = match guess.trim().parse() {
	    Ok(num) => num,
	    Err(_) => continue,
	};
	println!("Your guess was: {guess}.\n");
	
	match guess.cmp(&rng) {
	    Ordering::Less => println!("That's too low. The secret number is greater than that!\n"),
	    Ordering::Greater => println!("That's too high. The secret number is less than that!\n"),
	    Ordering::Equal => {
		println!("You got it! Congratulations!!!");
		break;
	    }
	}
    }
}

