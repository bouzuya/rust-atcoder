use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
    };
    let mut swap = false;
    let mut rows = (0..n).collect::<Vec<usize>>();
    let mut cols = (0..n).collect::<Vec<usize>>();
    for _ in 0..q {
        input! {
            t: usize,
        }
        match t {
            1 => {
                input! {
                    a: Usize1,
                    b: Usize1,
                }
                if swap {
                    cols.swap(a, b);
                } else {
                    rows.swap(a, b);
                }
            }
            2 => {
                input! {
                    a: Usize1,
                    b: Usize1,
                }
                if swap {
                    rows.swap(a, b);
                } else {
                    cols.swap(a, b);
                }
            }
            3 => {
                swap = !swap;
            }
            4 => {
                input! {
                    a: Usize1,
                    b: Usize1,
                }
                let (a, b) = if swap { (b, a) } else { (a, b) };
                println!("{}", rows[a] * n + cols[b]);
            }
            _ => unreachable!(),
        }
    }
}
