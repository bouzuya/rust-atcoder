use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let mut zero = 0;
    let mut a = a.into_iter().collect::<VecDeque<_>>();
    loop {
        while let Some(x) = a.pop_back() {
            if x == zero {
                continue;
            } else {
                a.push_back(x);
                break;
            }
        }

        if a.is_empty() {
            println!("Yes");
            return;
        }

        if let Some(x) = a.pop_front() {
            if x != zero {
                println!("No");
                return;
            }
        }
        zero = if zero == 0 { 1 } else { 0 };
    }
}
