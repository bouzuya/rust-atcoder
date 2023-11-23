use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        p: [Usize1; n],
    };
    let mut ans = vec![n; n];
    for (i, p_i) in p.iter().copied().enumerate() {
        ans[p_i] = i;
    }
    for a in ans {
        println!("{}", a + 1);
    }
}
