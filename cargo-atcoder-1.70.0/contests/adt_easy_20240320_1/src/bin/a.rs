use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let ans = ["ACE", "BDF", "CEG", "DFA", "EGB", "FAC", "GBD"].contains(&s.as_str());
    println!("{}", if ans { "Yes" } else { "No" });
}
