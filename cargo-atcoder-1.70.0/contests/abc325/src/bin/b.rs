use proconio::input;

fn main() {
    input! {
        n: usize,
        wx: [(usize, usize); n],
    };
    let mut max = 0;
    for t in 0..24 {
        let mut count = 0_usize;
        for (w, x) in wx.iter().copied() {
            let l = (t + x) % 24;
            if (9..18).contains(&l) {
                count += w;
            }
        }
        max = max.max(count);
    }
    let ans = max;
    println!("{}", ans);
}
