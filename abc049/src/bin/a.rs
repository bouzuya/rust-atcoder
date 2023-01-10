use proconio::input;

fn main() {
    input! {
        c: char,
    };
    let ans = "aeiou".chars().any(|x| x == c);
    println!("{}", if ans { "vowel" } else { "consonant" });
}
