use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    };
    let sum_a = a.iter().copied().sum::<usize>();
    if sum_a <= k {
        let mut ans = 0_usize;
        for a_i in a {
            ans += (1 + a_i) * a_i / 2;
        }
        println!("{}", ans);
        return;
    }

    let max_a = *a.iter().max().unwrap();

    let mut ok = max_a;
    let mut ng = 0;
    while ok - ng > 1 {
        let mid = ng + (ok - ng) / 2;
        let mut cnt = 0_usize;
        for a_i in a.iter().copied() {
            cnt += a_i.saturating_sub(mid);
        }
        if cnt <= k {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    let mut cnt = 0_usize;
    let mut sum = 0_usize;
    for a_i in a.iter().copied() {
        let ok = ok + 1;
        if a_i < ok {
            continue;
        }
        cnt += a_i + 1 - ok;
        sum += (ok + a_i) * (a_i + 1 - ok) / 2;
    }

    sum += ok * (sum_a.min(k) - cnt);
    println!("{}", sum);
}
