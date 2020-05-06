use proconio::input;

fn main() {
    input! {
        n: usize,
        mut s: [u64; n],
    };
    s.sort();
    let mut ans = s.iter().sum::<u64>();
    if ans % 10 == 0 {
        let mut ok = false;
        for i in 0..n {
            if s[i] % 10 != 0 {
                ans -= s[i];
                ok = true;
                break;
            }
        }
        if !ok {
            ans = 0;
        }
    }
    println!("{}", ans);
}
