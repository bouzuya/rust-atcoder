use proconio::input;

fn main() {
    input! {
        s: String,
    };
    let ans = s.ends_with('T');
    println!("{}", if ans { "YES" } else { "NO" });
}
