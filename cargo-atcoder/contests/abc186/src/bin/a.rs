use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
    };
    let ans = n / w;
    println!("{}", ans);
}
