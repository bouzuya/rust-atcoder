use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        mut abcde: [usize; 5],
    };
    abcde.sort();
    let mut min = 1 << 60;
    loop {
        let mut now = 0;
        for (i, x) in abcde.iter().copied().enumerate() {
            now += x;
            if i == 5 - 1 {
                break;
            }
            if now % 10 != 0 {
                now += 10 - now % 10;
            }
        }
        min = min.min(now);
        if !abcde.next_permutation() {
            break;
        }
    }

    let ans = min;
    println!("{}", ans);
}
