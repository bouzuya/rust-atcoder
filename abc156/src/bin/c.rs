use proconio::input;

fn main() {
    input! {
        n: usize,
        xv: [i32; n],
    };
    let ans = (1..=100)
        .map(|p| xv.iter().map(|x| (x - p).pow(2)).sum::<i32>())
        .min()
        .unwrap();
    println!("{}", ans);
}
