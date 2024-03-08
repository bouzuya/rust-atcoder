use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    };
    let mut a = vec![0_usize; n];
    for (i, p_i) in p.iter().copied().enumerate() {
        for j in -1_i64..=1 {
            let k = (n as i64 + p_i as i64 - i as i64 + j) as usize % n;
            a[k] += 1;
        }
    }
    let ans = a.iter().copied().max().unwrap();
    println!("{}", ans);
}
