use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
        h: u64,
    };
    let ans = (a + b) * h / 2;
    println!("{}", ans);
}
