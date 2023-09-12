use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    };
    let mut mex = vec![true; n + 1];
    let mut count = vec![0; n];
    for i in 0..m {
        mex[a[i]] = false;
        count[a[i]] += 1;
    }
    let mut min = mex
        .iter()
        .copied()
        .enumerate()
        .position(|(_, b)| b)
        .unwrap();
    for i in m..n {
        mex[a[i]] = false;
        count[a[i]] += 1;
        count[a[i - m]] -= 1;
        if count[a[i - m]] == 0 {
            mex[a[i - m]] = true;
            min = min.min(a[i - m]);
        }
    }
    let ans = min;
    println!("{}", ans);
}
