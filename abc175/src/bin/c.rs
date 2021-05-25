use proconio::input;

fn main() {
    input! {
        x: i64,
        k: i64,
        d: i64,
    };
    let x = x.abs();
    let k1 = x / d;
    if k <= k1 {
        println!("{}", x - d * k);
        return;
    }
    let ans = (if (k - k1) % 2 == 0 {
        x - d * k1
    } else {
        x - d * (k1 + 1)
    })
    .abs();
    println!("{}", ans);
}
