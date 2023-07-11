use proconio::input;

fn main() {
    input! {
        n: usize,
        st: [(String, i64); n],
        x: String,
    };
    let mut sleep = false;
    let mut sum = 0;
    for (s_i, t_i) in st.iter() {
        if s_i == &x {
            sleep = true;
            continue;
        }
        if sleep {
            sum += t_i;
        }
    }
    let ans = sum;
    println!("{}", ans);
}
