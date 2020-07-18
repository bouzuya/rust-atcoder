use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    // 8
    // 1, 3, 5, 7
    // 2, 2, 2, 2
    // 7
    // 0, 2, 4, 6
    // 1, 2, 2, 2
    let mut c = vec![0; n];
    for &a_i in a.iter() {
        c[a_i] += 1;
    }
    if n % 2 == 0 {
        for (i, &c_i) in c.iter().enumerate() {
            if c_i == 0 {
                continue;
            }
            if c_i == 2 && i % 2 != 0 {
                continue;
            }
            println!("{}", 0);
            return;
        }
        let mut ans = 1;
        for _ in 0..n / 2 {
            ans *= 2;
            ans %= 1_000_000_007;
        }
        println!("{}", ans);
    } else {
        for (i, &c_i) in c.iter().enumerate() {
            if c_i == 0 {
                continue;
            }
            if c_i == 1 && i == 0 {
                continue;
            }
            if c_i == 2 && i % 2 == 0 {
                continue;
            }
            println!("{}", 0);
            return;
        }
        let mut ans = 1;
        for _ in 0..n / 2 {
            ans *= 2;
            ans %= 1_000_000_007;
        }
        println!("{}", ans);
    }
}
