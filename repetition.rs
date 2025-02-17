#[derive(Debug)]
enum Day {
    Twelve,
    Eleven,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    One,
}

impl Day {
    fn as_string(&self) -> String {
        match *self {
            Day::Twelve => "12th day of Christmas",
            Day::Eleven => "11th day of Christmas",
            Day::Ten => "10th day of Christmas",
            Day::Nine => "9th day of Christmas",
            Day::Eight => "8th day of Christmas",
            Day::Seven => "7th day of Christmas",
            Day::Six => "6th day of Christmas",
            Day::Five => "5th day of Christmas",
            Day::Four => "4th day of Christmas",
            Day::Three => "3rd day of Christmas",
            Day::Two => "2nd day of Christmas",
            Day::One => "1st day of Christmas",
        }
        .to_string()
    }
}

fn lullaby(array: &[String], starting_day: usize) -> String {
    let mut pinphrase = String::from("On the ");
    
    // Start the iteration from the starting_day (counting backward)
    for i in (starting_day..12).rev() {
        let day = match i {
            11 => Day::Twelve,
            10 => Day::Eleven,
            9 => Day::Ten,
            8 => Day::Nine,
            7 => Day::Eight,
            6 => Day::Seven,
            5 => Day::Six,
            4 => Day::Five,
            3 => Day::Four,
            2 => Day::Three,
            1 => Day::Two,
            0 => Day::One,
            _ => panic!("Invalid day"),
        };
        
        // Construct the string for the current day
        let day_string = day.as_string();
        pinphrase.push_str(&day_string);  // Add the day string (e.g., "12th day of Christmas")
        
        // Add the parody element of that day
        pinphrase.push_str(&format!(": {}\n", array[i]));

        // Add the next part (call previous days)
        if i != 0 {
            pinphrase.push_str("and ");
        }
    }
    
    pinphrase
}

fn main() {
    // Convert array of string slices (&str) to Vec<String>
    let christmas_parody_array = vec![
        "one goat is bleating".to_string(),
        "two rings are exchanged".to_string(),
        "three sports cars turning".to_string(),
        "four lambdas solving".to_string(),
        "five engineers thinking".to_string(),
        "six buddhists praying".to_string(),
        "seven whales are wailing".to_string(),
        "eight aliens phishing".to_string(),
        "nine women singing".to_string(),
        "10 dragons puffing".to_string(),
        "11 Suns are shining".to_string(),
        "12 Angels playing".to_string(),
    ];

    // Call the lullaby starting from the 6th day
    let result = lullaby(&christmas_parody_array, 5); // Passing index 5 means we start at the 6th day

    // Output the full parody
    println!("{}", result);
}
