use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize
    };
    let x = n % k;
    let ans = std::cmp::min(x, if k > x { k - x } else { x - k });
    println!("{}", ans);
}
