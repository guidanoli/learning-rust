// Problem posed on Chapter 8-3
// Link: https://doc.rust-lang.org/book/ch08-03-hash-maps.html
// Convert strings to pig latin. The first consonant of each word is moved to
// the end of the word and “ay” is added, so “first” becomes “irst-fay.”
// Words that start with a vowel have “hay” added to the end instead
// (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!

fn to_pig_latin(s: &str) -> Option<String> {
    let mut chars = s.chars();
    return match chars.next() {
        None => None,
        Some(first_char) => match first_char {
            'a' | 'e' | 'i' | 'o' | 'u' => Some(format!("{}-hay", s)),
            _ => Some(format!("{}-{}ay", chars.as_str(), first_char)),
        }
    }
}

fn main() {
    assert_eq!(to_pig_latin("first"), Some(String::from("irst-fay")));
    assert_eq!(to_pig_latin("apple"), Some(String::from("apple-hay")));
}
