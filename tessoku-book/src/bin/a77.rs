use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        k: usize,
        mut a: [usize; n],
    };

    a.push(l);

    let mut b = vec![];
    let mut prev = 0_usize;
    for a_i in a.iter().copied() {
        b.push(a_i - prev);
        prev = a_i;
    }

    let mut ok = 1;
    let mut ng = l;
    while ng - ok > 1 {
        let x = ok + (ng - ok) / 2;

        let mut ys = vec![];
        let mut cur = 0_usize;
        for b_i in b.iter().copied() {
            cur += b_i;
            if cur >= x {
                ys.push(cur);
                cur = 0;
            }
        }
        if cur > 0 {
            let l = ys.len();
            ys[l - 1] += cur;
        }
        if ys.len() > k {
            ok = x;
        } else {
            ng = x;
        }
    }

    let ans = ok;
    println!("{}", ans);
}
