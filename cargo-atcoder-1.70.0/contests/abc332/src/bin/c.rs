use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        m: usize,
        s: Chars,
    };

    let mut ans = 0_usize;
    let mut used_m = 0_usize;
    let mut used_t = 0_usize;
    for s_i in s {
        match s_i {
            '0' => {
                used_m = 0;
                used_t = 0;
            }
            '1' => {
                if used_m < m {
                    used_m += 1;
                } else {
                    used_t += 1;
                }
            }
            '2' => {
                used_t += 1;
            }
            _ => unreachable!(),
        }
        ans = ans.max(used_t);
    }
    println!("{}", ans);
}
