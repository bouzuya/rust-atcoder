use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    };
    let ans = if s == t {
        "same"
    } else if s.eq_ignore_ascii_case(&t) {
        "case-insensitive"
    } else {
        "different"
    };
    println!("{}", ans);
}
