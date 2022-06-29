use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [usize; n],
    };
    let mut count = 0;
    let mut prev = 0;
    for h_i in h.iter().copied() {
        if h_i >= prev {
            count += 1;
            prev = h_i;
        }
    }
    let ans = count;
    println!("{}", ans);
}
