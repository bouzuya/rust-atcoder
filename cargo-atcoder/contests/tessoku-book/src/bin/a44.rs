use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
    };
    let mut a = (1..=n).collect::<Vec<usize>>();
    let mut rev = false;
    for _ in 0..q {
        input! {
            t: usize
        }
        match t {
            1 => {
                input! {
                    x: Usize1,
                    y: usize,
                }
                a[if rev { n - 1 - x } else { x }] = y;
            }
            2 => {
                rev = !rev;
            }
            3 => {
                input! {
                    x: Usize1,
                }
                println!("{}", a[if rev { n - 1 - x } else { x }]);
            }
            _ => unreachable!(),
        }
    }
}
