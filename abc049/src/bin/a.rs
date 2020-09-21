use proconio::input;

fn main() {
    input! {
        c: char,
    };
    let ans = match c {
        'a' | 'i' | 'u' | 'e' | 'o' => true,
        _ => false,
    };
    println!("{}", if ans { "vowel" } else { "consonant" });
}
