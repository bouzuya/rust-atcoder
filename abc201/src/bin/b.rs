use proconio::input;

fn main() {
    input! {
        n: usize,
        mut st: [(String, i64); n],
    };
    st.sort_by_key(|(_, t_i)| t_i.clone());
    let ans = st[st.len() - 2].clone();
    println!("{}", ans.0);
}
