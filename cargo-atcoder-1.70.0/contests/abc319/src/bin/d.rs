use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        l: [usize; n],
    };
    let mut ok = 1_000_000_000_000_000_000;
    let mut ng = l.iter().copied().max().unwrap() - 1;
    while ok - ng > 1 {
        let mid = (ok + ng) / 2;
        let mut count = 1_usize;
        let mut sum = 0_usize;
        for l_i in l.iter().copied() {
            sum += l_i;
            if sum == mid {
                count += 1;
                sum = 0;
                continue;
            }
            sum += 1;
            if sum == mid {
                count += 1;
                sum = 0;
                continue;
            }
            if sum > mid {
                count += 1;
                if l_i == mid {
                    count += 1;
                    sum = 0;
                } else {
                    sum = l_i + 1;
                }
            }
        }
        if sum == 0 {
            count -= 1;
        }
        if count <= m {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    let ans = ok;
    println!("{}", ans);
}
