fn is_lowercase_latin(c: char) -> bool {
    if c >= 'a' && c <= 'z' {true} else {false}
}

fn main() {
    loop {
        let mut word = String::new();
        std::io::stdin().read_line(&mut word).unwrap();
        word = word.trim().to_string();
        if word.chars().all(is_lowercase_latin) {
            const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
            if VOWELS.contains(&word.chars().next().unwrap()) {
                word = word + "ay";
            }
            else {
                loop {
                    word = format!("{}{}", &word[1..], &word[0..1]);
                    if VOWELS.contains(&word.chars().next().unwrap()) {
                        word = word + "hay";
                        break;
                    }
                }
            }
            println!("{}", word);
        }
        else {
            println!("'{}' contains undue characters", word);   
        }
    }
}
