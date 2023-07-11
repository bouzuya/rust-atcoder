use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
    };
    let mut c = vec![0_u64; m];
    for _ in 0..n {
        input! {
            k_i: usize,
            a: [Usize1; k_i],
        };
        for &a_i in a.iter() {
            c[a_i] += 1;
        }
    }
    let ans = c.iter().filter(|&&count| count == n as u64).count();
    println!("{}", ans);
}
