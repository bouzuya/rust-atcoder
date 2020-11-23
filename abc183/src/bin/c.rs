use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        k: u64,
        t: [[u64; n]; n],
    };
    let mut count = 0;
    let mut is = (1..n).collect::<Vec<usize>>();
    loop {
        let mut sum = 0_u64;
        let mut i = 0;
        for &j in is.iter() {
            sum += t[i][j];
            i = j;
        }
        sum += t[i][0];
        if sum == k {
            count += 1;
        }
        if !is.next_permutation() {
            break;
        }
    }
    let ans = count;
    println!("{}", ans);
}
