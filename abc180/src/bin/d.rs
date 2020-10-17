use proconio::input;

fn main() {
    input! {
        x: i64,
        y: i64,
        a: i64,
        b: i64,
    };
    let ans = {
        let mut c = 0_i64;
        let mut n = x;
        while n.checked_mul(a).is_some() && n * a < n + b && n * a < y {
            n *= a;
            c += 1;
        }
        c + ((y - 1 - n) / b)
    };
    println!("{}", ans);
}
