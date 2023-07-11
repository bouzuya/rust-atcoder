use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    };
    let mut words = vec![];
    let mut word = vec![];
    for c in s {
        if c.is_uppercase() {
            if word.is_empty() {
                word.push(c);
            } else {
                word.push(c);
                words.push(word.iter().collect::<String>());
                word.clear();
            }
        } else {
            word.push(c);
        }
    }
    words.push(word.iter().collect::<String>());

    words.sort_by_key(|s| s.to_lowercase());
    for w in words {
        print!("{}", w);
    }
    println!();
}
