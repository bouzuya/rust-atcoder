use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        q: usize,
        lrt: [(Usize1, Usize1, i64); q],
    };
    let mut a = vec![0_i64; n];
    for &(l_i, r_i, t_i) in lrt.iter() {
        for j in l_i..=r_i {
            a[j] = t_i;
        }
    }
    for &a_i in a.iter() {
        println!("{}", a_i);
    }
}
