use proconio::input;

fn main() {
    input! {
        n: usize
    };
    let ans = n / 10 + n % 10;
    println!("{}", ans);
}
