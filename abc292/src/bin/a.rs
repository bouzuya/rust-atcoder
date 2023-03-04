use proconio::input;

fn main() {
    input! {
        s: String,
    };
    let ans = s.to_ascii_uppercase();
    println!("{}", ans);
}
