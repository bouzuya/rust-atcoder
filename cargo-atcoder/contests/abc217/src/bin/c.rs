use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        p: [Usize1; n],
    };
    let mut q = vec![n; n];
    for (i, p_i) in p.iter().copied().enumerate() {
        q[p_i] = i;
    }
    for (i, q_i) in q.iter().enumerate() {
        print!("{}{}", q_i + 1, if i == n - 1 { "\n" } else { " " });
    }
}
