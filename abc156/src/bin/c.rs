use proconio::input;

fn main() {
    input! {
        n: usize,
        xv: [i32; n],
    };
    let mut ans = 1_000_000;
    for i in 0..=100 {
        let mut s = 0;
        for &x in xv.iter() {
            s += (x - i).pow(2);
        }
        ans = std::cmp::min(ans, s);
    }
    println!("{}", ans);
}
