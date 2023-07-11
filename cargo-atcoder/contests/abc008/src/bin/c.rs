use proconio::input;

fn main() {
    input! {
        n: usize,
        mut c: [usize; n],
    };
    let mut s = vec![];
    for (i, c_i) in c.iter().copied().enumerate() {
        let mut s_i = 0;
        for (j, c_j) in c.iter().copied().enumerate() {
            if i == j {
                continue;
            }
            if c_i % c_j == 0 {
                s_i += 1;
            }
        }
        s.push(s_i);
    }

    let mut ans = 0_f64;
    for s_i in s {
        if s_i % 2 == 0 {
            let s_i = s_i as f64;
            ans += (s_i + 2_f64) / (2_f64 * s_i + 2_f64);
        } else {
            ans += 1_f64 / 2_f64;
        }
    }

    println!("{}", ans);
}
