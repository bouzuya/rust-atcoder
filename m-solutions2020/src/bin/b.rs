use proconio::input;

fn main() {
    input! {
        a: i64,
        mut b: i64,
        mut c: i64,
        k: i64,
    };

    let mut m = 0;
    while a >= b {
        b *= 2;
        m += 1;
    }
    while b >= c {
        c *= 2;
        m += 1;
    }
    let ans = m <= k;
    println!("{}", if ans { "Yes" } else { "No" });
}
