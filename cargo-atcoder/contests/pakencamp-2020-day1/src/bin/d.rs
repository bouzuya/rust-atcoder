use proconio::input;

fn main() {
    input! {
        n: i64,
        k: i64,
    };
    let ans = if k <= n {
        k.pow(3)
    } else if k <= 2 * n {
        k.pow(3) - 3 * (k - n).pow(3)
    } else if k <= 3 * n {
        n.pow(3) * 6 - (3 * n - k).pow(3)
    } else {
        n.pow(3) * 6
    };
    println!("{}", ans);
}
