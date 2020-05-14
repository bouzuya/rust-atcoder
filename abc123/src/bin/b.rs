use proconio::input;
use superslice::*;

fn main() {
    input! {
        abcde: [i64; 5],
    };
    let inf = 130 * 5_i64;
    let mut ans = inf;
    let mut index = (0..abcde.len()).collect::<Vec<usize>>();
    loop {
        let mut t = 0;
        for &i in index.iter() {
            t += if t % 10 == 0 { 0 } else { 10 - t % 10 };
            t += abcde[i];
        }
        ans = std::cmp::min(ans, t);
        if !index.next_permutation() {
            break;
        }
    }
    println!("{}", ans);
}
