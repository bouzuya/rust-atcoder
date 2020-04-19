use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        vv: [i64; n],
    };
    let mut ans = 0;
    for i in 0..=std::cmp::min(n, k) {
        let i_l = i;
        for j in i..=std::cmp::min(n, k) {
            let i_r = n - (j - i);
            let mut cv: Vec<i64> = vv[..i_l]
                .iter()
                .map(|&x| x)
                .chain(vv[i_r..].iter().map(|&x| x))
                .collect();
            cv.sort();
            let mut sum = cv.iter().sum();
            ans = std::cmp::max(ans, sum);
            for (l, &v) in cv.iter().enumerate() {
                if l >= k - j {
                    break;
                }
                sum -= v;
                ans = std::cmp::max(ans, sum);
            }
        }
    }
    println!("{}", ans);
}
