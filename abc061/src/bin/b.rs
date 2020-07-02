use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    };
    let mut c = vec![0; n];
    for &(a_i, b_i) in ab.iter() {
        c[a_i] += 1;
        c[b_i] += 1;
    }
    for &c_i in c.iter() {
        println!("{}", c_i);
    }
}
