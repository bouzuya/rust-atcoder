use proconio::input;

fn main() {
    input! {
        k: usize,
        s: String
    };
    let ans = if s.len() <= k {
        s
    } else {
        s[0..k].to_owned() + "..."
    };
    println!("{}", ans);
}
