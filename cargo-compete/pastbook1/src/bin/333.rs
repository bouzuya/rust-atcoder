use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut s = VecDeque::new();
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
                s.push_back((c, x));
            }
            2 => {
                input! {
                    d: usize,
                }

                let mut sum_x = 0_usize;
                let mut count = vec![0_usize; 26];
                while let Some((c, x)) = s.pop_front() {
                    sum_x += x;
                    if sum_x < d {
                        count[(c as u8 - b'a') as usize] += x;
                    } else {
                        let y = sum_x - d;
                        count[(c as u8 - b'a') as usize] += x - y;
                        s.push_front((c, y));
                        break;
                    }
                }
                let ans = count.iter().fold(0_usize, |acc, x| acc + x * x);
                println!("{}", ans);
            }
            _ => unreachable!(),
        }
    }
}
