use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [char; n],
    };
    let ans = if s.contains(&'Y') { "Four" } else { "Three" };
    println!("{}", ans);
}
