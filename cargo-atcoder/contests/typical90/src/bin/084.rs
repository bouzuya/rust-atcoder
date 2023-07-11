use proconio::{input, marker::Chars};
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        s: Chars,
    };
    let mut o = vec![];
    let mut x = vec![];
    for (i, s_i) in s.iter().copied().enumerate() {
        match s_i {
            'o' => o.push(i),
            'x' => x.push(i),
            _ => unreachable!(),
        }
    }

    let mut count = 0;
    for (i, s_i) in s.iter().copied().enumerate() {
        let j = match s_i {
            'o' => {
                let j = x.lower_bound(&(i + 1));
                if j == x.len() {
                    n
                } else {
                    x[j]
                }
            }
            'x' => {
                let j = o.lower_bound(&(i + 1));
                if j == o.len() {
                    n
                } else {
                    o[j]
                }
            }
            _ => unreachable!(),
        };
        count += n - j;
    }
    let ans = count;
    println!("{}", ans);
}
