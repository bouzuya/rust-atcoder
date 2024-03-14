use proconio::input;

fn main() {
    input! {
        m: usize,
        d: [usize; m],
    };
    let mid = (d.iter().sum::<usize>() + 1) / 2;
    let mut sum = 0_usize;
    for (i, d_i) in d.iter().copied().enumerate() {
        if sum + d_i < mid {
            sum += d_i;
            continue;
        }
        println!("{} {}", i + 1, mid - sum);
        return;
    }
}
