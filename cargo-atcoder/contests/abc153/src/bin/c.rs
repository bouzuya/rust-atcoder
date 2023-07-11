use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut h: [usize; n],
    };
    h.sort();
    h.reverse();
    let ans = h.into_iter().skip(k).sum::<usize>();
    println!("{}", ans);
}
