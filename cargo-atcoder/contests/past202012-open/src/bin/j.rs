use proconio::input;
use proconio::marker::{Chars, Usize1};

#[derive(Debug)]
enum Op {
    C(char),
    D(usize),
}

fn main() {
    input! {
        s: Chars,
        x: Usize1,
    };
    let mut stack = vec![];
    let mut l = 0_usize;
    for &c in s.iter() {
        if c.is_digit(10) {
            let d = (c as u8 - '0' as u8) as usize;
            l += l * d;
            stack.push((l, Op::D(d)));
        } else {
            l += 1;
            stack.push((l, Op::C(c)));
        }
        if x < l {
            break;
        }
    }

    let mut index = x;
    while let Some((l, op)) = stack.pop() {
        match op {
            Op::C(c) => {
                if index + 1 == l {
                    println!("{}", c);
                    return;
                } else {
                    continue;
                }
            }
            Op::D(_) => {
                let (l, _) = *stack.last().unwrap();
                index %= l;
            }
        }
    }

    unreachable!("bug");
}
