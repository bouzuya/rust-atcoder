use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let mut os = vec![0; 60];
    for a_i in a.iter().copied() {
        for j in 0..60 {
            os[j] += (a_i >> j) & 1;
        }
    }
    let mut max = 0_usize;
    for a_i in a.iter().copied() {
        let mut sum = 0_usize;
        for j in 0..60 {
            let a_ij = (a_i >> j) & 1;
            let o = os[j];
            let z = n - os[j];
            sum += if a_ij == 0 { o } else { z } * 2_usize.pow(j as u32);
        }
        max = max.max(sum);
    }
    let ans = a.iter().sum::<usize>().max(max);
    println!("{}", ans);
}
