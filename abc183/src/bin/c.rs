use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        k: usize,
        t: [[usize; n]; n],
    };

    let mut count = 0_usize;
    let mut is = (1..n).collect::<Vec<usize>>();
    loop {
        let mut time = 0_usize;
        let mut prev = 0_usize;
        for next in is.iter().copied() {
            time += t[prev][next];
            prev = next;
        }
        time += t[prev][0];

        if time == k {
            count += 1;
        }

        if !is.next_permutation() {
            break;
        }
    }

    let ans = count;
    println!("{}", ans);
}
