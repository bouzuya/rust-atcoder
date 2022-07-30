use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    };

    let mut ok = *a.iter().max().unwrap() * 10;
    let mut ng = 0;
    while ok - ng > 1 {
        let mid = ng + (ok - ng) / 2;

        let mut count = 0;
        for a_i in a.iter().copied() {
            if a_i * 10 == mid {
                continue;
            }
            count += a_i * 10 / mid;
        }

        if count <= k {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    let ans = (ok + 10 - 1) / 10;
    println!("{}", ans);
}
