use proconio::input;

fn main() {
    input! {
        t: usize,
        n: usize,
        p: [[usize; n]; t],
    };
    let mut max = vec![0; n];
    for k in 0..t {
        let mut sum = 0_usize;
        for j in 0..n {
            max[j] = max[j].max(p[k][j]);
            sum += max[j];
        }
        println!("{}", sum);
    }
}
