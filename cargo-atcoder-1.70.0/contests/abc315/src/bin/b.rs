use proconio::input;

fn main() {
    input! {
        m: usize,
        d: [usize; m],
    };
    let sum = d.iter().copied().sum::<usize>();
    let mut cur = 0_usize;
    for (i, d_i) in d.iter().copied().enumerate() {
        cur += d_i;
        if cur * 2 >= sum {
            println!("{} {}", i + 1, (sum + 1) / 2 - (cur - d_i));
            return;
        }
    }
}
