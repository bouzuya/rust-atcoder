use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn main() {
    input! {
        s: Chars,
        q: usize,
    }
    let mut rev = false;
    let mut deq = VecDeque::from(s);
    for _ in 0..q {
        input! { t: usize }
        match t {
            1 => rev = !rev,
            2 => {
                input! { f: usize, c: char }
                let to_push_front = match f {
                    1 => !rev,
                    2 => rev,
                    _ => unreachable!(),
                };
                if to_push_front {
                    deq.push_front(c)
                } else {
                    deq.push_back(c)
                }
            }
            _ => unreachable!(),
        }
    }
    let ans: String = if !rev {
        deq.iter().collect()
    } else {
        deq.iter().rev().collect()
    };
    println!("{}", ans);
}
