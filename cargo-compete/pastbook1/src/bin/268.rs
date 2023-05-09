use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    // let ans = a
    //     .iter()
    //     .copied()
    //     .position(|a_i| a_i >= k)
    //     .map(|i| i as i64)
    //     .unwrap_or(-1);
    // println!("{}", ans);

    if a[n - 1] < k {
        println!("-1");
        return;
    }
    if a[0] >= k {
        println!("0");
        return;
    }

    let mut ok = n - 1;
    let mut ng = 0;
    while ok - ng > 1 {
        let mid = ng + (ok - ng) / 2;
        if a[mid] >= k {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    let ans = ok;
    println!("{}", ans);
}
