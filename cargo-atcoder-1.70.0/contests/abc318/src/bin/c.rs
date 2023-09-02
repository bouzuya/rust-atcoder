use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        p: usize,
        mut f: [usize; n],
    };

    f.sort();
    f.reverse();

    let mut index = 0_usize;
    let mut sum = f.iter().copied().sum::<usize>();
    let mut min = sum;
    for i in 0..n {
        sum += p;
        for j in index..n.min(index + d) {
            sum -= f[j];
        }
        index += d;
        min = min.min(sum);
        if i * d > n {
            break;
        }
    }
    let ans = min;
    println!("{}", ans);
}
