use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        st: [(String, String); n],
    };
    let set = st.into_iter().collect::<HashSet<(String, String)>>();
    let ans = set.len() != n;
    println!("{}", if ans { "Yes" } else { "No" });
}
