use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let ans = n.saturating_sub(1);
    println!("{}", ans);
}
