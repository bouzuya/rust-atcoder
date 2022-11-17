use std::collections::VecDeque;

use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        n: usize,
        x: Usize1,
        mut a: Chars,
    };

    let mut queue = VecDeque::new();
    queue.push_back(x);
    a[x] = '@';
    while let Some(pos) = queue.pop_front() {
        if pos > 0 && a[pos - 1] == '.' {
            a[pos - 1] = '@';
            queue.push_back(pos - 1);
        }
        if pos < n - 1 && a[pos + 1] == '.' {
            a[pos + 1] = '@';
            queue.push_back(pos + 1);
        }
    }
    println!("{}", a.into_iter().collect::<String>());
}
