use proconio::input;

fn main() {
    input! {
        a: i64,
        mut b: i64,
        mut c: i64,
        k: i64,
    };

    let mut is_b = a >= b;
    for _ in 0..k {
        if is_b {
            b *= 2;
            is_b = a >= b;
        } else {
            c *= 2;
        }
    }
    let ans = a < b && b < c;
    println!("{}", if ans { "Yes" } else { "No" });
}
