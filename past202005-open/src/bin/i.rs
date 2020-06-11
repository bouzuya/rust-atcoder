use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        q: usize,
    };
    let mut pr = (0..n).collect::<Vec<_>>();
    let mut pc = (0..n).collect::<Vec<_>>();
    let mut is_ij_rc = true;
    for _ in 0..q {
        input! { t: usize };
        match t {
            1 => {
                input! { r_a: Usize1, r_b: Usize1 };
                if is_ij_rc {
                    pr.swap(r_a, r_b);
                } else {
                    pc.swap(r_a, r_b);
                }
            }
            2 => {
                input! { c_a: Usize1, c_b: Usize1 };
                if is_ij_rc {
                    pc.swap(c_a, c_b);
                } else {
                    pr.swap(c_a, c_b);
                }
            }
            3 => {
                is_ij_rc = !is_ij_rc;
            }
            4 => {
                input! { r_a: Usize1, c_b: Usize1 };
                let (r, c) = if is_ij_rc {
                    (pr[r_a], pc[c_b])
                } else {
                    (pr[c_b], pc[r_a])
                };
                let ans = n * r + c;
                println!("{}", ans);
            }
            _ => unreachable!(),
        }
    }
}
