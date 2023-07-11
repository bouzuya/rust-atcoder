use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        r: usize,
        c: usize,
        k: usize,
        n: usize,
        rcs: [(Usize1, Usize1); n],
    };
    let mut xss = vec![vec![]; r];
    let mut cs = vec![0_usize; c];
    for &(r_i, c_i) in rcs.iter() {
        xss[r_i].push(c_i);
        cs[c_i] += 1;
    }
    let mut ccc = vec![0_usize; n + 1];
    for &c in cs.iter() {
        ccc[c] += 1;
    }
    let mut count = 0;
    for xs in xss.iter() {
        let cr = xs.len();
        if cr > k {
            continue;
        }
        let mut sum = ccc[k - cr];
        for &x in xs.iter() {
            if cs[x] == k - cr {
                sum -= 1;
            }
            if cs[x] == k - cr + 1 {
                sum += 1;
            }
        }
        count += sum;
    }
    let ans = count;
    println!("{}", ans);
}
