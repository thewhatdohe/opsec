use rand::{RngExt};

fn main() {
    let mut rng = rand::rng();
    let jokes = [
        "Rule #1 of opsec: never talk about opsec",
        "The most secure system is the one that's powered off, unplugged, and buried in concrete",
        "There is no patch for human stupidity",
        "I don't always test in production, but when I do, I don't tell anyone",
        "Social engineering: because why hack the firewall when you can just ask nicely",
        "The best encryption is handwriting nobody can read",
        "opsec level: I whisper my password even when I'm alone",
        "Nothing says 'secure' like a Post-it note of the WiFi password visible from the street",
        "Zero trust architecture: I don't even trust myself with sudo",
        "My threat model includes my own future self forgetting the password",
        "Why did the computer keep freezing? Because it left its Windows open!",
        "The only unhackable system is the one that doesn't exist yet",
        "Did you try turning it off and on again?",
        "I have no idea what I'm doing",
        "Hook, Line, and Sinker",
        "No, I will not fix your computer"
    ];
    let index: usize = rng.random_range(0..jokes.len());
    println!("{}", jokes[index]);
    
}