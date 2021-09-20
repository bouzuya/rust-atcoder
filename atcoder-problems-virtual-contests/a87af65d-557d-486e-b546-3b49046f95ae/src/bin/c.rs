use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    if (n != m) && (n.max(m) - n.min(m) > 1) {
        println!("0");
        return;
    }

    let p = 1_000_000_007_usize;
    let ans = if n == m {
        let mut s = 1_usize;
        for i in 0..n {
            s *= n - i;
            s %= p;
            s *= m - i;
            s %= p;
        }
        s *= 2;
        s %= p;
        s
    } else {
        let (n, m) = (n.max(m), n.min(m));
        let mut s = 1_usize;
        for i in 0..n {
            s *= n - i;
            s %= p;
            if i != n - 1 {
                s *= m - i;
                s %= p;
            }
        }
        s
    };
    println!("{}", ans);
}
