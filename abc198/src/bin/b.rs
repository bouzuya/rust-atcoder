use proconio::input;

fn main() {
    input! {
        n: String,
    };
    let s = n.trim_end_matches('0').to_owned();
    let t = s.chars().rev().collect::<String>();
    let ans = s == t;
    println!("{}", if ans { "Yes" } else { "No" });
}
