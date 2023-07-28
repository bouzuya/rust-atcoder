use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n + 1],
        b: [usize; n],
    };
    let mut count = 0_usize;
    for (i, b_i) in b.iter().copied().enumerate() {
        let x_i0 = a[i].min(b_i);
        a[i] -= x_i0;

        let x_i1 = a[i + 1].min(b_i - x_i0);
        a[i + 1] -= x_i1;

        count += x_i0 + x_i1;
    }
    let ans = count;
    println!("{}", ans);
}
