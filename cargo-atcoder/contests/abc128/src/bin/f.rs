use proconio::input;

fn main() {
    input! {
        n: usize,
        sv: [i64; n],
    };
    // a <= b は終了できないため a > b
    // 0 < b < a <= n - 1
    // c = a - b とする
    let mut ans = 0_i64; // a = n - 1, b = * のとき
    for c in 1..=n - 1 {
        let mut sum = 0_i64;
        let mut set = std::collections::BTreeSet::new();
        for x in 0.. {
            let l = x * c;
            if l >= n {
                break;
            }
            let r = n - 1 - l;
            if !set.insert(l) || !set.insert(r) {
                break;
            }
            // b = a - c で b > 0
            let a = r;
            if a <= c {
                break;
            }
            sum += sv[l] + sv[r];
            ans = std::cmp::max(ans, sum);
        }
    }
    println!("{}", ans);
}
