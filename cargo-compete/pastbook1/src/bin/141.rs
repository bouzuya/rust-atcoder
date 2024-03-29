use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        mut c: [usize; n],
        q: usize,
    }

    let mut count = 0_usize;
    let mut all_count = 0_usize;
    let mut all_min = c.iter().copied().min().unwrap();
    let mut odd_count = 0_usize;
    let mut odd_min = c
        .iter()
        .copied()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, c_i)| c_i)
        .min()
        .unwrap();
    for _ in 0..q {
        input! {
            t: usize,
        }
        match t {
            1 => {
                input! {
                    x: Usize1,
                    a: usize,
                }
                let a_sum = a + all_count + if x % 2 == 0 { odd_count } else { 0 };
                if c[x] >= a_sum {
                    c[x] -= a;
                    all_min = all_min.min(c[x]);
                    if x % 2 == 0 {
                        odd_min = odd_min.min(c[x]);
                    }
                    count += a;
                }
            }
            2 => {
                input! {
                    a: usize,
                }
                if a <= odd_min {
                    odd_min -= a;
                    all_min = all_min.min(odd_min);
                    odd_count += a;
                    count += a * ((n + 1) / 2);
                }
            }
            3 => {
                input! {
                    a: usize,
                }
                if a <= all_min {
                    all_min -= a;
                    odd_min -= a;
                    all_count += a;
                    count += a * n;
                }
            }
            _ => unreachable!(),
        }
    }
    println!("{}", count);
}
