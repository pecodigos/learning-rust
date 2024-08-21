use std::io::{self, Write};

fn main() {
    let mut dates = Vec::new();

    loop {
        let mut input = String::new();
        
        print!("Enter a date in yyyy/mm/dd format. Otherwise, type 'done' to stop: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Failed to read line.");

        // Date parsing
        let input = input.trim();
        
        if input.to_lowercase() == "done" {
            break;
        }
    
        // Split the input string and colleect into a new Vector
        let parts: Vec<&str> = input.split('/').collect();

        // Check if each date has exactly three parts (y, m, d)
        if parts.len() == 3 {
            if let (Ok(year), Ok(month), Ok(day)) = (
                parts[0].parse::<u32>(),
                parts[1].parse::<u32>(),
                parts[2].parse::<u32>(),
            ) {
                dates.push((year, month, day));
            } else {
                println!("Invalid date format. Please try again.");
            }
        }
        else {
            println!("Please enter a date in the correct format!");
        }
    }
    let mut number = String::new();

    print!("\nEnter the month you want to check the number of occurences: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut number).expect("Failed to read line.");

    // Month parsing
    let number: u32 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    println!("\nNumber of dates in month {}: {}", number, number_in_month(&dates, number));
}

fn number_in_month(dates: &[(u32, u32, u32)], month: u32) -> u32 {
    if dates.is_empty() {
        0
    }
    else {
        let (_, m, _) = dates[0]; // Getting the first month of the list
        let tail = &dates[1..]; // Assigning tail of the list in a variable

        if m == month {
            number_in_month(tail, month) + 1
        }
        else
        {
            number_in_month(tail, month)
        }
    }
}