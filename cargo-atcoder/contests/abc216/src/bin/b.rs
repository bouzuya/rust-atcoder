use proconio::input;

fn main() {
    input! {
        n: usize,
        st: [(String, String); n],
    };
    for i in 0..n {
        for j in i + 1..n {
            let (s_i, t_i) = &st[i];
            let (s_j, t_j) = &st[j];
            if s_i == s_j && t_i == t_j {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
