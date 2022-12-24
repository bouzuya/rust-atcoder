use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let mut boxes = 0_usize;
    let mut stack = vec![];
    for (i, c) in s.iter().copied().enumerate() {
        match c {
            '(' => {
                stack.push((i, boxes));
            }
            ')' => {
                let (_, bits) = stack.pop().unwrap();
                boxes = bits;
            }
            'a'..='z' => {
                let index = (c as u8 - b'a') as usize;
                if (boxes & (1 << index)) != 0 {
                    println!("No");
                    return;
                }
                boxes |= 1 << index;
            }
            _ => unreachable!(),
        }
    }
    println!("Yes");
}
