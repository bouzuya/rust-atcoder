use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut ans = 0_usize;
    for l in 1.. {
        let mut b = 0_usize;
        for j in 0..l {
            b += 10_usize.pow(j as u32);
        }
        if b > n {
            break;
        }

        let mut t = b + 1;
        while b <= n {
            ans += n.min(t - 1) + 1 - b;
            b *= 10;
            t *= 10;
        }
    }
    println!("{}", ans);
}
