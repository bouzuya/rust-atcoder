use proconio::input;

fn main() {
    input! {
        r: usize,
    };
    let ans = 3 * r.pow(2);
    println!("{}", ans);
}
