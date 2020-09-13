use proconio::input;

fn main() {
    input! {
        x: usize,
    };
    let ans = if x == 0 { 1 } else { 0 };
    println!("{}", ans);
}
