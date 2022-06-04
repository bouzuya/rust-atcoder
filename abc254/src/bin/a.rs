use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let ans = n % 100;
    println!("{:02}", ans);
}
