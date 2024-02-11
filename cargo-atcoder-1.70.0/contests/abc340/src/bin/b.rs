use proconio::{input, marker::Usize1};

fn main() {
    input! {
        q: usize,
    };
    let mut a = vec![];
    for _ in 0..q {
        input! {
            t: usize
        }
        match t {
            1 => {
                input! {
                    x: usize,
                }
                a.push(x);
            }
            2 => {
                input! {
                    k: Usize1,
                }
                println!("{}", a[a.len() - 1 - k]);
            }
            _ => unreachable!(),
        }
    }
}
