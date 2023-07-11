use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [usize; n],
    };
    let mut t = vec![false; n];
    for a in 1..=1000 {
        for b in 1..=1000 {
            for (i, s_i) in s.iter().copied().enumerate() {
                if 4 * a * b + 3 * a + 3 * b == s_i {
                    t[i] = true;
                }
            }
        }
    }
    let ans = t.iter().filter(|&&x| !x).count();
    println!("{}", ans);
}
