use proconio::input;

fn main() {
    input! {
        s: String,
    };
    let ws = vec!["Sunny", "Cloudy", "Rainy"];
    let p = ws.iter().position(|w| w == &s).unwrap();
    let ans = ws[(p + 1) % ws.len()];
    println!("{}", ans);
}
