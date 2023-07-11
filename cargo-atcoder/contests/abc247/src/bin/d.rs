use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        q: usize,
    };
    let mut deque = VecDeque::new();
    for _ in 0..q {
        input! { t: usize }
        match t {
            1 => {
                input! { x: usize, c: usize }
                deque.push_back((x, c));
            }
            2 => {
                input! { mut c: usize }
                let mut sum = 0_usize;
                while let Some((x, x_c)) = deque.pop_front() {
                    if x_c >= c {
                        sum += x * c;
                        deque.push_front((x, x_c - c));
                        break;
                    } else {
                        sum += x * x_c;
                        c -= x_c;
                    }
                }
                println!("{}", sum);
            }
            _ => unreachable!(),
        }
    }
}
