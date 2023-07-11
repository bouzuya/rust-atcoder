use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        p: usize,
        q: usize,
        r: usize,
        xyz: [(Usize1, Usize1, usize); r],
    };
    let mut max = 0_usize;
    for nset in 0_usize..(1 << n) {
        if nset.count_ones() as usize > p {
            continue;
        }
        let mut mscore = vec![0_usize; m];
        for (x, y, z) in xyz.iter().copied() {
            if (nset & (1 << x)) == 0 {
                continue;
            }
            mscore[y] += z;
        }
        mscore.sort();
        mscore.reverse();
        max = max.max(mscore[0..q].iter().sum::<usize>());
    }
    let ans = max;
    println!("{}", ans);
}
