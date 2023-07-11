use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        c: [String; n],
        d: [String; m],
        p: [usize; m + 1],
    };
    let ans = c
        .iter()
        .map(|c_i| match d.iter().position(|d_j| c_i == d_j) {
            Some(j) => p[j + 1],
            None => p[0],
        })
        .sum::<usize>();
    println!("{}", ans);
}
