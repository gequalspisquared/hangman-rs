use std::io;

/*
 *
 *  .----.
 *  |    |
 *  |    O
 *  |   /|\
 *  |   / \
 *  | 
 *  *------
 *
 */

fn draw(secret_word: &str, guesses: &[char]) {
    clear_screen();
    draw_hangman(num_incorrect_guesses(&secret_word, &guesses));
    println!();
    draw_guesses(&secret_word, &guesses);
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn num_incorrect_guesses(secret_word: &str, guesses: &[char]) -> usize {
    guesses.iter().fold(0, |sum, c| {
        if secret_word.contains(*c) {
            sum
        } else {
            sum + 1
        }
    })
}

fn num_correct_guesses(secret_word: &str, guesses: &[char]) -> usize {
    guesses.len() - num_incorrect_guesses(&secret_word, &guesses)
}

fn draw_hangman(num_incorrect: usize) {
    let art = [
        "|       \n".to_owned() +
        "|       \n"            +
        "|         ",

        "|    O  \n".to_owned() +
        "|       \n"            +
        "|         ",

        "|    O  \n".to_owned() +
        "|    |  \n"            +
        "|         ",

        "|    O  \n".to_owned() +
        "|   /|  \n"            +
        "|         ",

        "|    O  \n".to_owned() +
        "|   /|\\\n"            +
        "|         ",
        
        "|    O  \n".to_owned() +
        "|   /|\\\n"            +
        "|   /     ",

        "|    O  \n".to_owned() +
        "|   /|\\\n"            +
        "|   / \\  ",
    ];

    println!(".----.");
    println!("|    |");
    println!("{}", art[num_incorrect]);
    println!("|     ");
    println!("*------");
}

fn draw_guesses(secret_word: &str, guesses: &[char]) {
    let mut correct: String = String::from("");
    let mut incorrect: String = String::from("");

    for c in secret_word.chars() {
        if guesses.contains(&c) {
            correct.push(c);
        } else {
            correct += " ";
        }
    }

    for c in guesses.iter() {
        if !secret_word.contains(*c) {
            incorrect.push(*c);
        } 
    }

    if incorrect.len() > 0 {
        println!("{}   incorrect: {}", correct, incorrect);
    } else {
        println!("{}", correct);
    }
    println!("{:-<1$}", "", secret_word.len());
}

fn get_valid_guess() -> Option<char>{
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to get guess");

    match guess.trim().to_lowercase().parse::<char>() {
        Ok(c) => {
            if c.is_alphabetic() {
                Some(c)
            } else {
                None
            }
        },
        Err(_) => None
    }
}

fn count_unique_chars(word: &str) -> usize {
    let mut unique_chars: Vec<char> = vec![];

    for c in word.chars() {
        if !unique_chars.contains(&c) {
            unique_chars.push(c);
        }
    }

    unique_chars.len()
}

fn main() {
    let mut guesses: Vec<char> = vec![];
    let mut warning: String = String::from("");
    let secret_word = random_word::gen();
    let unique_chars: usize = count_unique_chars(&secret_word);

    while num_incorrect_guesses(&secret_word, &guesses) < 6 &&
          num_correct_guesses(&secret_word, &guesses) != unique_chars {
        draw(&secret_word, &guesses);
        if warning.len() > 0 {
            println!("{}", warning);
            warning = String::from("");
        }
        
        println!("Please enter your guess: ");
        if let Some(guess) = get_valid_guess() {
            if guesses.contains(&guess) {
                warning = String::from("You've already guessed that letter!");
                continue;
            }
            guesses.push(guess);
            println!("Your guess was: {}", guess);
            println!("All guesses: {:?}", guesses);
        } else {
            println!("Please enter a valid character");
            continue;
        }
    }

    draw(&secret_word, &guesses);

    if num_correct_guesses(&secret_word, &guesses) == unique_chars {
        println!("Congratulations, you won!");
    } else {
        println!("You lost :/");
    }
}
