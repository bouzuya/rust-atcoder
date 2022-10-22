use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    };
    let mut depth = vec![0_usize; 2 * n + 1];
    for (i, a_i) in a.iter().copied().enumerate() {
        depth[2 * i + 1] = depth[a_i] + 1;
        depth[2 * i + 2] = depth[a_i] + 1;
    }
    for d in depth {
        println!("{}", d);
    }
}
