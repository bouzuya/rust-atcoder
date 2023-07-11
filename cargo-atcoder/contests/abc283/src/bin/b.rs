use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        q: usize,
    };
    for _ in 0..q {
        input! {
            t: usize,
            k: Usize1,
        }
        match t {
            1 => {
                input! {
                    x: usize,
                }
                a[k] = x;
            }
            2 => {
                println!("{}", a[k]);
            }
            _ => unreachable!(),
        }
    }
}
