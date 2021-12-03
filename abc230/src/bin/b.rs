use proconio::input;

fn main() {
    input! {
        s: String,
    };
    let ans = vec!["oxxoxxoxxo", "xxoxxoxxox", "xoxxoxxoxx"]
        .iter()
        .any(|&x| x.contains(&s));
    println!("{}", if ans { "Yes" } else { "No" });
}
