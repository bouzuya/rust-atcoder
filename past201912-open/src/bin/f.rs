use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    };
    let mut w = vec![];
    let mut r = None;
    for (i, &c) in s.iter().enumerate() {
        if c.is_uppercase() {
            r = match r {
                None => Some(i),
                Some(j) => {
                    w.push(s[j..i + 1].iter().collect::<String>());
                    None
                }
            };
        }
    }
    w.sort_by_key(|w_i| w_i.to_lowercase());
    for w_i in w.iter() {
        print!("{}", w_i);
    }
    println!();
}
