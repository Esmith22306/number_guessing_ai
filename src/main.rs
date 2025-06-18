use std::io;

struct AIPlayer {
    low: i32,
    high: i32,
    guess: i32,
    tries: u32,
}

impl AIPlayer {
    fn new(low: i32, high: i32) -> Self {
        let guess = (low + high) / 2;
        AIPlayer {
            low,
            high,
            guess,
            tries: 1,
        }
    }

    fn make_guess(&self) -> i32 {
        self.guess
    }

    fn update(&mut self, feedback: &str) {
        match feedback {
            "too low" => self.low = self.guess + 1,
            "too high" => self.high = self.guess - 1,
            _ => {}
        }

        self.guess = (self.low + self.high) / 2;
        self.tries += 1;
    }
}

fn main() {
    println!("Think of a number between 1 and 100, and I'll try to guess it!");
    println!("Respond with 'too high', 'too low', or 'correct'.");

    let mut ai = AIPlayer::new(1, 100);

    loop {
        let guess = ai.make_guess();
        println!("Is it {}?", guess);

        let mut feedback = String::new();
        io::stdin().read_line(&mut feedback).expect("Failed to read input");
        let feedback = feedback.trim().to_lowercase();

        if feedback == "correct" {
            println!("Yay! I guessed it in {} tries!", ai.tries);
            break;
        } else if feedback == "too high" || feedback == "too low" {
            ai.update(&feedback);
        } else {
            println!("Please respond with 'too high', 'too low', or 'correct'.");
        }
    }
}
