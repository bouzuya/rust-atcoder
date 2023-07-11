use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let ans = if n < 1000 { "ABC" } else { "ABD" };
    println!("{}", ans);
}
