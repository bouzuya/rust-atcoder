use proconio::input;

fn main() {
    input! {
        s: String,
    };
    let t = "oxx".repeat(5);
    let ans = t.contains(&s);
    println!("{}", if ans { "Yes" } else { "No" });
}
