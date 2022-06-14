use proconio::input;

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            _m: usize,
            xy: [(i64, i64); n],
        }

        let mut ans = -(1_i64 << 60);
        let mut a = 0;
        let mut b = 0;
        for (x_i, y_i) in xy.iter().copied() {
            let a_l = a + b + x_i;
            let a_r = a + b * y_i + x_i * y_i * (y_i + 1) / 2;
            if x_i < 0 && b > 0 {
                let j = (b / -x_i).min(y_i);
                let a_m = a + b * j + x_i * j * (j + 1) / 2;
                ans = ans.max(a_m);
            }
            ans = ans.max(a_l).max(a_r);
            a = a_r;
            b += x_i * y_i;
        }

        println!("{}", ans);
    }
}
