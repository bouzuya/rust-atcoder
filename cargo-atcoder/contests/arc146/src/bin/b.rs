use proconio::input;

fn f(a: &[usize], k: usize, x: usize) -> usize {
    let mut b = vec![];
    for a_i in a.iter().copied() {
        let b_i = if a_i < x {
            x - a_i
        } else {
            let mut xmask = if x.is_power_of_two() {
                (x << 1) - 1
            } else {
                x.next_power_of_two() - 1
            };
            let mut a2 = a_i & xmask;
            let mut x2 = x & xmask;
            let mut c = 0_usize;
            for j in (0..40).rev() {
                let bit = 1 << j;
                if (x2 & bit != 0) && (a2 & bit == 0) {
                    let d = x2 - a2;
                    c += d;
                    a2 += d;
                }

                xmask &= xmask ^ bit;
                x2 &= xmask;
                a2 &= xmask;
            }
            c
        };
        b.push(b_i);
    }
    b.sort();
    b.into_iter().take(k).sum::<usize>()
}

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        a: [usize; n]
    };
    let mut ok = 0_usize;
    let mut ng = 1_usize << 40;
    while ng - ok > 1 {
        let x = ok + (ng - ok) / 2;
        if f(&a, k, x) <= m {
            ok = x;
        } else {
            ng = x;
        }
    }
    let ans = ok;
    println!("{}", ans);
}
