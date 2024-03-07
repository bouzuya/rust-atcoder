use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let ans = n.trailing_zeros();
    println!("{}", ans);
}
