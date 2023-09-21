use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        st: [(String, String); n],
    };
    let ans = st.into_iter().collect::<HashSet<_>>().len() != n;
    println!("{}", if ans { "Yes" } else { "No" });
}
