use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut cols = (0..n).collect::<Vec<usize>>();
    let mut rows = (0..n).collect::<Vec<usize>>();
    let mut rotate = false;
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
                if rotate {
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
                if rotate {
                    rows.swap(a, b);
                } else {
                    cols.swap(a, b);
                }
            }
            3 => {
                rotate = !rotate;
            }
            4 => {
                input! {
                    a: Usize1,
                    b: Usize1,
                }
                if rotate {
                    println!("{}", n * rows[b] + cols[a]);
                } else {
                    println!("{}", n * rows[a] + cols[b]);
                }
            }
            _ => unreachable!(),
        }
    }
}
