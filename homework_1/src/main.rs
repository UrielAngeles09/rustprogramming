const FREEZING_POINT_F: f64 = 32.0;

// Assignment 1 
fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING_POINT_F) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + FREEZING_POINT_F
}

fn assignment1() {
    let mut temp_f: i32 = 32;
    println!("Assignment 1: Temperature Converter");
    println!(
        "{}°F = {:.2}°C",
        temp_f,
        fahrenheit_to_celsius(temp_f as f64)
    );

    for _ in 0..5 {
        temp_f += 1;
        println!(
            "{}°F = {:.2}°C",
            temp_f,
            fahrenheit_to_celsius(temp_f as f64)
        );
    }
    println!("0°C = {:.2}°F", celsius_to_fahrenheit(0.0));
    println!();

}

// Assignment 2
fn assignment2() {
    fn is_even(n: i32) -> bool {
        n % 2 == 0
    }

    let numbers: [i32; 10] = [3, 5, 10, 15, 8, 12, 7, 20, 25, 30];
    println!("Assignment 2: Number Analyzer");

    for &num in numbers.iter() {
        if num % 15 == 0 {
            println!("{} -> FizzBuzz", num);
        } else if num % 3 == 0 {
            println!("{} -> Fizz", num);
        } else if num % 5 == 0 {
            println!("{} -> Buzz", num);
        } else if is_even(num) {
            println!("{} -> Even", num);
        } else {
            println!("{} -> Odd", num);
        }
    }

    let mut sum = 0;
    let mut i = 0;
    while i < numbers.len() {
        sum += numbers[i];
        i += 1;
    }
    println!("Sum of numbers = {}", sum);

    let mut largest = numbers[0];
    for &num in numbers.iter() {
        if num > largest {
            largest = num;
        }
    }
    println!("Largest number = {}", largest);
    println!();
}

// Assignment 3
fn assignment3() {
    fn check_guess(guess: i32, secret: i32) -> i32 {
        if guess == secret {
            0
        } else if guess > secret {
            1
        } else {
            -1
        }
    }

    let secret = 42;
    let guesses = [30, 50, 40, 42]; // simulate "user input"
    let mut attempts = 0;

    println!("Assignment 3: Guessing Game");
    for &guess in guesses.iter() {
        attempts += 1;
        let result = check_guess(guess, secret);
        if result == 0 {
            println!("Guess {}: {} is correct!", attempts, guess);
            break;
        } else if result == 1 {
            println!("Guess {}: {} is too high!", attempts, guess);
        } else {
            println!("Guess {}: {} is too low!", attempts, guess);
        }
    }

    println!("It took {} guesses to find the secret number.", attempts);
    println!();
}

fn main() {
    assignment1();
    assignment2();
    assignment3();
}
