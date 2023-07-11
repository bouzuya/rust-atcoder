use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let ans = format!("{:04}", n);
    println!("{}", ans);
}
