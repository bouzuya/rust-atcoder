use proconio::input;

fn main() {
    input! {
        s: String,
    };
    let ans = if s.ends_with("er") { "er" } else { "ist" };
    println!("{}", ans);
}
