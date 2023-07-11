use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        q: usize,
    };
    let mut rs = (0..n).collect::<Vec<usize>>();
    let mut cs = (0..n).collect::<Vec<usize>>();
    let mut f = false;
    for _ in 0..q {
        input! {
            t_i: usize,
        };
        match t_i {
            1 => {
                input! {
                    a_i: Usize1,
                    b_i: Usize1,
                };
                let t = rs[a_i];
                rs[a_i] = rs[b_i];
                rs[b_i] = t;
            }
            2 => {
                input! {
                    a_i: Usize1,
                    b_i: Usize1,
                };
                let t = cs[a_i];
                cs[a_i] = cs[b_i];
                cs[b_i] = t;
            }
            3 => {
                let t = cs;
                cs = rs;
                rs = t;
                f = !f;
            }
            4 => {
                input! {
                    a_i: Usize1,
                    b_i: Usize1,
                };
                let ans = if f {
                    rs[a_i] + cs[b_i] * n
                } else {
                    rs[a_i] * n + cs[b_i]
                };
                println!("{}", ans);
            }
            _ => unreachable!(),
        }
    }
}
