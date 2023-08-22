use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let ans = n.pow(3);
    println!("{}", ans);
}
