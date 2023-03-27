use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: [Chars; 3],
    };
    let mut s = vec![
        s[0].iter()
            .copied()
            .map(|c| (c as u8 - b'a') as usize)
            .collect::<VecDeque<usize>>(),
        s[1].iter()
            .copied()
            .map(|c| (c as u8 - b'a') as usize)
            .collect::<VecDeque<usize>>(),
        s[2].iter()
            .copied()
            .map(|c| (c as u8 - b'a') as usize)
            .collect::<VecDeque<usize>>(),
    ];
    let mut index = 0_usize;
    while let Some(c) = s[index].pop_front() {
        index = c;
    }
    let ans = (b'A' + index as u8) as char;
    println!("{}", ans);
}
