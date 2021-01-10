use proconio::input;
use proconio::marker::Chars;
use superslice::*;

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };
    let is: Vec<Vec<usize>> = {
        let mut is = vec![vec![]; 26];
        for (i, &c) in s.iter().enumerate() {
            is[(c as u8 - 'a' as u8) as usize].push(i);
        }
        is
    };
    let mut count = 0;
    let mut i = 0;
    for c in t {
        let i_c = (c as u8 - 'a' as u8) as usize;
        let js = &is[i_c];
        let j = js.lower_bound(&i);
        if j == js.len() {
            count += 1;
            if js.is_empty() {
                println!("-1");
                return;
            }
            i = js[0] + 1;
        } else {
            i = js[j] + 1;
        }
    }
    let ans = count * s.len() + i;
    println!("{}", ans);
}
