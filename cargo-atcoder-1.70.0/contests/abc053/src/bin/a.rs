use proconio::input;

fn main() {
    input! {
        x: usize,
    }
    let ans = if x < 1200 { "ABC" } else { "ARC" };
    println!("{}", ans);
}
