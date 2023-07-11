use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
    };
    let ans = n.pow(2) - a;
    println!("{}", ans);
}
