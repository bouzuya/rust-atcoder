use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    };
    let ans = t.starts_with(&s);
    println!("{}", if ans { "Yes" } else { "No" });
}
