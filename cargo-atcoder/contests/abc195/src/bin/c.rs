use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut count = 0_usize;
    for i in 0_usize..=5 {
        // [min, max] のとき i 桁
        let min = 10_usize.pow(3 * i as u32);
        let max = 10_usize.pow(3 * (i + 1) as u32) - 1;
        if n >= min {
            count += i * (n.min(max) - min + 1_usize);
        }
    }
    let ans = count;
    println!("{}", ans);
}
