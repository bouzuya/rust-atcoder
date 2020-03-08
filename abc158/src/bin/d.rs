use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn main() {
    input! {
        s: Chars,
        q: usize,
    }
    let mut r = false;
    let mut d = VecDeque::from(s);
    for _ in 0..q {
        input! { t: usize }
        match t {
            1 => r = !r,
            2 => {
                input! { f: usize, c: char }
                match f {
                    1 => {
                        if !r {
                            d.push_front(c)
                        } else {
                            d.push_back(c)
                        }
                    }
                    2 => {
                        if !r {
                            d.push_back(c)
                        } else {
                            d.push_front(c)
                        }
                    }
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }
    let ans: String = if !r {
        d.iter().collect()
    } else {
        d.iter().rev().collect()
    };
    println!("{}", ans);
}
