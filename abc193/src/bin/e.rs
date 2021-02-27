// サンプル通っていない
use proconio::input;

fn gcd(n: i64, m: i64) -> i64 {
    if n < m {
        gcd(m, n)
    } else if m == 0 {
        n
    } else {
        gcd(m, n % m)
    }
}

fn lcm(n: i64, m: i64) -> i64 {
    n * m / gcd(n, m)
}

fn main() {
    input! {
        t: usize,
        xypq: [(i64, i64, i64, i64); t],
    };
    for (x, y, p, q) in xypq {
        let c1 = x + y + x + y;
        let c2 = p + q;
        let l = lcm(c1, c2);
        let r1 = l / c1;
        let r2 = l / c2;
        // 最小公倍数
        if c1 == c2 && !(x <= p && p < x + y) {
            println!("infinity");
            continue;
        }
        println!("foo");
    }
}
