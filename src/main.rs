extern crate rayon;
use rayon::prelude::*;

fn main() {
    let plaintext = "abcdefghijklmnopqrstuvwxyz";
    let cyphertext = rotate(plaintext, 8);

    println!("plaintext: {}", plaintext);
    println!("cyphertext: {}", cyphertext);

    println!("Cracking...");

    let mut rotated = Vec::new();

    for amount in 0..26 {
        rotated.push(rotate(&cyphertext, amount));
    }

    let result = rotated.par_iter().enumerate().find(|&(_, text)| {
        text == "abcdefghijklmnopqrstuvwxyz"
    });

    if let Some((index, _)) = result {
        println!("Cracked! encrypted with {}, rotate {} times to break", 26 - index, index);
        println!("rotated: '{}'", rotate(&cyphertext, index as u8));
    } else {
        println!("Couldn't find a solution sorry");
    }

}

fn rotate(message: &str, amount: u8) -> String {
    let bytes = message.as_bytes()
        .into_iter()
        .fold(Vec::new(), |mut acc, &c| {
            acc.push(rotate_one(c, amount));
            acc
        });

    String::from_utf8(bytes).unwrap()
}

fn rotate_one(letter: u8, amount: u8) -> u8 {
    let lower_bound = 97;
    let upper_bound = 122;
    let range = upper_bound - lower_bound;

    if letter < lower_bound || letter > upper_bound {
        return letter;
    }

    let new = letter + amount;

    if new > upper_bound {
        new - range - 1
    } else {
        new
    }
}