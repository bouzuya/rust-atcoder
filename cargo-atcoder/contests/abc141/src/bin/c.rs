use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        q: usize,
        a: [Usize1; q],
    };
    let mut c = vec![0_usize; n];
    for a_i in a {
        c[a_i] += 1;
    }
    for c_i in c {
        let ans = (k + c_i).saturating_sub(q) > 0;
        println!("{}", if ans { "Yes" } else { "No" });
    }
}
