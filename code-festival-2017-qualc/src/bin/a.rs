use proconio::input;

fn main() {
    input! {
        s: String,
    };
    let ans = s.find("AC").is_some();
    println!("{}", if ans { "Yes" } else { "No" });
}
