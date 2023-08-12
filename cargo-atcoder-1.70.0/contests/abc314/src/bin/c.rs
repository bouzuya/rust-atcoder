use proconio::{
    input,
    marker::{Chars, Usize1},
};
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
        c: [Usize1; n],
    };

    let s2 = s
        .iter()
        .copied()
        .chain(s.iter().copied())
        .collect::<Vec<char>>();
    let c2 = c
        .iter()
        .copied()
        .chain(c.iter().copied())
        .collect::<Vec<usize>>();

    let mut index = vec![vec![]; m];
    for i in 0..n * 2 {
        index[c2[i]].push(i);
    }

    let mut ans = vec![];
    for i in n..n * 2 {
        let l = index[c2[i]].len();
        let j = index[c2[i]].lower_bound(&i);
        ans.push(s2[index[c2[i]][(j + l - 1) % l]]);
    }
    println!("{}", ans.iter().collect::<String>());
}
