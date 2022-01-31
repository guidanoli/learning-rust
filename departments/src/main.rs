// Problem posed on Chapter 8-3
// Link: https://doc.rust-lang.org/book/ch08-03-hash-maps.html
// Using a hash map and vectors, create a text interface to allow a user to
// add employee names to a department in a company. For example, “Add Sally
// to Engineering” or “Add Amir to Sales.” Then let the user retrieve a
// list of all people in a department or all people in the company by
// department, sorted alphabetically.

use std::io::{self, Write};
use std::collections::HashMap;

const GREETINGS: &str = "DEPARTMENT DATABASE APPLICATION
Allowed commands:
- Add [name] to [department]
- Show [department]";

fn main() {
    let mut db: HashMap<String, Vec<String>> = HashMap::new();

    println!("{}", GREETINGS);

    loop {
        print!("> ");
        let _ = io::stdout().flush();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        let mut words = input.trim().split_whitespace();

        match words.next() {
            Some(command) => match command {
                "Add" => {
                    let mut name = match words.next() {
                        None => {
                            eprintln!("Expected name");
                            continue;
                        },
                        Some(name) => String::from(name),
                    };

                    let mut found_to = false;

                    loop {
                        match words.next() {
                            None => break, // not expected
                            Some(word) => match word {
                                "to" => {
                                    found_to = true;
                                    break;
                                },
                                _ => {
                                    name.push_str(" ");
                                    name.push_str(word);
                                },
                            },
                        };
                    }

                    if !found_to {
                        eprintln!("Syntax: Add [name] to [deparment]");
                        continue;
                    }

                    let department = match words.next() {
                        None => {
                            eprintln!("Expected department");
                            continue;
                        },
                        Some(department) => String::from(department),
                    };

                    let department = words.fold(department, |res, word| format!("{} {}", res, word));

                    let employees = db.entry(department).or_insert(Vec::new());
                    (*employees).push(name);
                },
                "Show" => {
                    let department = match words.next() {
                        None => {
                            eprintln!("Expected department");
                            continue;
                        },
                        Some(department) => String::from(department),
                    };

                    let department = words.fold(department, |res, word| format!("{} {}", res, word));

                    if let Some(employees) = db.get(&department) {
                        for name in employees {
                            println!("{}", name);
                        }
                    }
                },
                _ => {
                    eprintln!("Unknown command");
                    continue;
                },
            },
            None => break,
        }
    }
}
