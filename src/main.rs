fn main() {
    let plaintext = "hello";
    let cyphertext = rotate(plaintext, 8);

    println!("plaintext: {}", plaintext);
    println!("cyphertext: {}", cyphertext);

    println!("Cracking...");

    let mut rotated = Vec::new();

    for amount in 0..26 {
        rotated.push(rotate(&cyphertext, amount));
    }

    for (index, text) in rotated.into_iter().enumerate() {
        if text == "hello" {
            println!("Cracked! encrypted with {}, rotate {} times to break", 25 - index, index + 1)
        }
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
    let upper_bound = 123;
    let range = upper_bound - lower_bound;

    let new = letter + amount;

    if new > upper_bound {
        new - range
    } else {
        new
    }
}