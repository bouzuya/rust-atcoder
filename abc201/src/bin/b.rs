use proconio::input;

fn main() {
    input! {
        n: usize,
        mut st: [(String, usize); n],
    };
    st.sort_by_key(|(_, t)| *t);
    st.reverse();
    let ans = &st[1].0;
    println!("{}", ans);
}
