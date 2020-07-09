use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        mut s: [Chars; n],
    };
    let mut max_len = 0;
    for s_i in s.iter_mut() {
        s_i.sort();
        max_len = std::cmp::max(max_len, s_i.len());
    }
    let mut ans = vec![];
    let mut j = vec![0; n];
    for o in 0..26 {
        let c = ('a' as u8 + o as u8) as char;
        let mut min_l = max_len;
        for (i, j_i) in j.iter_mut().enumerate() {
            let mut l = 0;
            while *j_i < s[i].len() && s[i][*j_i] == c {
                l += 1;
                *j_i += 1;
            }
            min_l = std::cmp::min(min_l, l);
        }
        for _ in 0..min_l {
            ans.push(c);
        }
    }
    println!("{}", ans.iter().collect::<String>());
}
