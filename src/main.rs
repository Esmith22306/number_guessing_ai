use std::io;
/// Brings in rusts standard library in/output module so we can read user inputs from terminal

/// Struct representing the AI player with guessing state
/// Keeps track of everything the AI will need
struct AIPlayer {
    low: i32,              // Current lower bound
    high: i32,             // Current upper bound
    guess: i32,            // Current guess
    tries: u32,            // Number of attempts made
    history: Vec<i32>,     // History of all guesses
}

/// Implmenting AIPlayer methods
impl AIPlayer {
    /// Creates a new AIPlayer instance with starting bounds
    fn new(low: i32, high: i32) -> Self {
        let guess = (low + high) / 2; // Initial guess
        AIPlayer {
            low,
            high,
            guess,
            tries: 1,
            history: vec![guess], // Start history with first guess
        }
    }

    /// Returns the current guess
    fn make_guess(&self) -> i32 {
        self.guess
    }

    /// Updates the AI's guess range based on feedback
    /// Creates new lower or upper bound
    fn update(&mut self, feedback: &str) {
        match feedback {
            "too low" => self.low = self.guess + 1,
            "too high" => self.high = self.guess - 1,
            _ => {}
        }

        // Calculate the next guess
        self.guess = (self.low + self.high) / 2;
        self.tries += 1;
        self.history.push(self.guess); // Save to guess history
    }

    /// Prints the full guess history
    fn print_history(&self) {
        println!("Guess history: {:?}", self.history);

        // Use slicing to print last 3 guesses
        let recent = &self.history[self.history.len().saturating_sub(3)..];
        println!("Last 3 guesses: {:?}", recent);
    }
}

/// Helper function to read a valid integer from user input
fn read_bound(prompt: &str) -> i32 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if let Ok(num) = input.trim().parse::<i32>() {
            return num;
        }

        println!("Please enter a valid integer.");
    }
}

fn main() {
    
    // Get custom range from user
    println!("Welcome to the Number Guessing AI!");
    let low = read_bound("Enter the lower bound:");
    let high = read_bound("Enter the upper bound:");

    println!("Think of a number between {} and {}.", low, high);
    println!("I will try to guess it. Please respond with 'too low', 'too high', or 'correct'.");

    let mut ai = AIPlayer::new(low, high);

    /// Game loop
    loop {
        let guess = ai.make_guess();
        println!("Is it {}?", guess);

        let mut feedback = String::new();
        io::stdin().read_line(&mut feedback).expect("Failed to read input");
        let feedback = feedback.trim().to_lowercase();

        if feedback == "correct" {
            println!("Yay! I guessed your number in {} tries!", ai.tries);
            ai.print_history(); // Show guess history at the end
            break;
        } else if feedback == "too high" || feedback == "too low" {
            ai.update(&feedback);
        } else {
            println!("Please respond with 'too high', 'too low', or 'correct'.");
        }
    }
}
