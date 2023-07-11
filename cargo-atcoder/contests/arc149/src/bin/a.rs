use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    };

    let mut ans = (0, 0);
    for d in 1..=9 {
        let mut maxlen = 0;
        let mut x = 0;
        for len in 1..=n {
            x = ((10 * x) % m + d) % m;
            if x == 0 {
                maxlen = maxlen.max(len);
            }
        }
        if maxlen > 0 && maxlen >= ans.0 {
            ans = (maxlen, d);
        }
    }
    if ans == (0, 0) {
        println!("-1");
    } else {
        for _ in 0..ans.0 {
            print!("{}", ans.1);
        }
        println!();
    }
}
