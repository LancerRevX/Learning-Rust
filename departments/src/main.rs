use std::collections::{HashMap, hash_map::Entry};

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        let mut request = String::new();
        std::io::stdin().read_line(&mut request).unwrap();
        let words: Vec<&str> = request.split_whitespace().collect();
        match (words.get(0), words.len()) {
            (Some(&"print"), 1) => {
                let mut departments: Vec<_> = company.keys().collect();
                departments.sort();
                for department in departments {
                    println!("{}: {:?}", department, company[department]);
                }
            },
            (Some(&"printunsorted"), 1) => {
                println!("{:?}", company);
            },
            (Some(&"print"), 2) => {
                match company.get(words[1]) {
                    Some(staff) => println!("{:?}", staff),
                    _ => println!("No {} department found", words[1])
                }
            },
            (Some(&"add"), 4) if words[2] == "to" => {
                match company.entry(String::from(words[3])) {
                    Entry::Occupied(mut entry) => {
                        entry.get_mut().push(String::from(words[1]));
                    },
                    Entry::Vacant(entry) => {
                        entry.insert(vec![String::from(words[1])]);
                    }
                }
            },
            (Some(&"exit"), 1) => {
                println!("Good bye!");
                break;
            }
            (None, _) => println!("Type something!"),
            _ => println!("Invalid syntax!")
        }
    }
}
