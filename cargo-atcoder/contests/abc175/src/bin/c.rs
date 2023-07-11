use proconio::input;

fn main() {
    input! {
        x: i64,
        k: i64,
        d: i64,
    };
    let x = x.abs();
    let count = x / d;
    if k <= count {
        println!("{}", x - k * d);
        return;
    }

    let x = x - count * d;
    let k = k - count;
    let ans = (x - (k % 2) * d).abs();
    println!("{}", ans);
}
