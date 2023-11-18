use proconio::{input, marker::Chars};

macro_rules! chmax {
    ($max_v: expr, $v: expr) => {
        if $v > $max_v {
            $max_v = $v;
            true
        } else {
            false
        }
    };
}

fn main() {
    input! {
        _n: usize,
        s: Chars,
    };
    let mut set = vec![0_usize; 26];
    let mut prev = vec![];
    for s_i in s {
        match prev.last() {
            Some(c) => {
                if *c == s_i {
                    prev.push(s_i);
                } else {
                    prev = vec![s_i];
                }
            }
            None => {
                prev = vec![s_i];
            }
        }
        chmax!(set[(prev[0] as u8 - b'a') as usize], prev.len());
    }
    let ans = set.into_iter().sum::<usize>();
    println!("{}", ans);
}
