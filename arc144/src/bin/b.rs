use proconio::input;

fn main() {
    input! {
        n: usize,
        a: i64,
        b: i64,
        capital_a: [i64; n],
    };
    let mut ok = capital_a.iter().copied().min().unwrap();
    let mut ng = capital_a.iter().copied().max().unwrap() + 1;
    while (ng - ok) > 1 {
        let mid = ok + (ng - ok) / 2;
        let mut count = 0_i64;
        for a_i in capital_a.iter().copied() {
            count += match a_i.cmp(&mid) {
                std::cmp::Ordering::Less => ((mid - a_i) + a - 1) / a,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => ((mid - a_i) / b),
            }
        }
        if count <= 0 {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    let ans = ok;
    println!("{}", ans);
}
