use proconio::input;

fn main() {
    input! {
        x: i64,
        k: i64,
        d: i64
    };
    let c = x.abs() / d;
    let ans = if k <= c {
        (x.abs() - k * d).abs()
    } else {
        let x = x.abs() % d;
        if (k - c) % 2 == 0 {
            x
        } else {
            (x - d).abs()
        }
    };
    println!("{}", ans);
}
