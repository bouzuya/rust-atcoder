use proconio::{input, marker::Chars};
use superslice::Ext;

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };

    let mut pos = vec![vec![]; 26];
    for (i, s_i) in s.repeat(2).into_iter().enumerate() {
        pos[((s_i as u8 - b'a') as usize)].push(i);
    }

    let mut prev = None;
    let mut cur = 0;
    for t_i in t {
        let ps = &pos[((t_i as u8 - b'a') as usize)];
        if ps.is_empty() {
            println!("-1");
            return;
        }
        match prev {
            Some(index) => {
                let next = ps[ps.lower_bound(&(index + 1))];
                cur += next - index;
                prev = Some(next % s.len());
            }
            None => {
                prev = Some(ps[0]);
                cur = ps[0] + 1;
            }
        }
    }

    let ans = cur;
    println!("{}", ans);
}
