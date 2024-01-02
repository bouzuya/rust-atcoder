use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        q: usize,
    };
    let mut deque = VecDeque::new();
    for _ in 0..q {
        input! {
            t: usize,
        }
        match t {
            1 => {
                input! {
                    c: char,
                    x: usize,
                }
                deque.push_back((c, x));
            }
            2 => {
                input! {
                    mut d: usize,
                }
                let mut del = vec![0_usize; 26];
                while let Some((c, x)) = deque.pop_front() {
                    if x <= d {
                        del[(c as u8 - b'a') as usize] += x;
                        d -= x;
                    } else {
                        del[(c as u8 - b'a') as usize] += d;
                        deque.push_front((c, x - d));
                        break;
                    }
                }
                println!("{}", del.iter().map(|del_i| del_i.pow(2)).sum::<usize>());
            }
            _ => unreachable!(),
        }
    }
}
