use proconio::input;

fn main() {
    input! {
        a: String,
        b: String,
    };
    let ans = if a.len() < b.len() { b } else { a };
    println!("{}", ans);
}
