use std::collections::HashMap;
use std::io;
use std::convert::TryFrom;

use rand::seq::SliceRandom;
use rand::thread_rng;

const NUM_LENGTH: usize = 4;

fn generate_secret() -> HashMap<usize, usize> {
  let mut result = HashMap::new();
  let mut seq: [usize; 9] = [0; 9];
  let mut rng = thread_rng();
  for i in 0..seq.len() {
    seq[i] = i;
  }
  seq.shuffle(&mut rng);

  for i in 0..NUM_LENGTH {
    result.insert(seq[i], i);
  }

  return result;
}

fn generate_response(secret: &HashMap<usize, usize>, guess: u16) -> String {
  let mut in_place = 0;
  let mut existing = 0;
  for (i, ch) in guess.to_string().chars().enumerate() {
    let el = ch.to_digit(10).unwrap();
    let usize_el = usize::try_from(el).unwrap();
    match secret.get(&usize_el) {
      Some(&num) => {
        if num == i {
          in_place += 1;
        } else {
          existing += 1;
        }
      }
      _ => {},
    }
  }
  return format!("{}A{}B", in_place, existing);
}

fn main() {
    println!("Guess the number");

    let secret_hash = generate_secret();

    println!("The secret number is: {:?}", secret_hash);

    loop {
      println!("Please input your guess:");
      let mut guess = String::new();

      io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

      let guess: u16 = match guess.trim().parse() {
        Ok(num) => num,
        Err(err) => {
          println!("Err {}", err);
          continue;
        },
      };

      println!(
        "You guessed {}, response: {}", guess, generate_response(&secret_hash, guess)
      );
    }

}
