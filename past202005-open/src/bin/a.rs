use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    };
    let ans = if s == t {
        "same"
    } else if s.to_lowercase() == t.to_lowercase() {
        "case-insensitive"
    } else {
        "different"
    };
    println!("{}", ans);
}
