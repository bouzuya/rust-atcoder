use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        r: usize,
    };

    let mut ans = 0_usize;
    for i in 0..64 {
        if (n & (1 << i)) == 0 {
            continue;
        }

        let u = (2_usize.pow((i + 1) as u32) - 1).min(r);
        let l = 2_usize.pow(i as u32).max(l);
        if u < l {
            continue;
        }
        ans += u - l + 1;
    }

    println!("{}", ans);
}
