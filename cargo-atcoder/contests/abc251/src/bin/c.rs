use std::{cmp::Reverse, collections::HashMap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        st: [(String, usize); n],
    };
    let mut map = HashMap::new();
    for (i, (s_i, t_i)) in st.iter().enumerate() {
        map.entry(s_i).or_insert((*t_i, i + 1));
    }
    let mut os = vec![];
    for (_, v) in map {
        os.push(v);
    }
    os.sort_by_key(|&(t_i, i)| (Reverse(t_i), i));

    let ans = os[0].1;
    println!("{}", ans);
}
