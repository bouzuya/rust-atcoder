use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let ans = n / 2 + if n % 2 == 0 { 0 } else { 3 };
    println!("{}", ans);
}
