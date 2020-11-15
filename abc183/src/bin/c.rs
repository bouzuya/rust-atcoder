use proconio::input;
use superslice::*;

fn main() {
    input! {
        n: usize,
        k: u64,
        t: [[u64; n]; n],
    };
    let mut count = 0;
    let mut d = (1..n).collect::<Vec<usize>>();
    loop {
        let mut c = 0;
        let mut sum = 0;
        for &d_i in d.iter() {
            sum += t[c][d_i];
            c = d_i;
        }
        sum += t[c][0];
        if sum == k {
            count += 1;
        }
        if !d.next_permutation() {
            break;
        }
    }
    println!("{}", count);
}
