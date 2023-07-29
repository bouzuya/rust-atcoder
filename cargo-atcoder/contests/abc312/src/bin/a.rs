use proconio::input;

fn main() {
    input! {
        s: String,
    };
    let ans = vec!["ACE", "BDF", "CEG", "DFA", "EGB", "FAC", "GBD"]
        .iter()
        .any(|t| t == &s);
    println!("{}", if ans { "Yes" } else { "No" });
}
