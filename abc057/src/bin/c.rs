use proconio::input;

fn main() {
    input! {
        n: i64
    };

    if n == 1 {
        println!("{}", 1);
        return;
    }
    let mut ans = 1_000_000_000_000_000_i64;
    for a in 1..n {
        if a * a > n {
            break;
        }
        if n % a == 0 {
            let mut b = n / a;
            let mut c = 0;
            while b > 0 {
                b /= 10;
                c += 1;
            }
            ans = std::cmp::min(ans, c);
        }
    }
    println!("{}", ans);
}
