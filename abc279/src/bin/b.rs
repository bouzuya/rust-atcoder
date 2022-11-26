use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    };
    let ans = s.contains(&t);
    println!("{}", if ans { "Yes" } else { "No" });
}
