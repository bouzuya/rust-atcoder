use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let mut count = 0_usize;
    for (i, a_i) in a.iter().copied().enumerate() {
        let i = i + 1;
        if i % 2 != 0 && a_i % 2 != 0 {
            count += 1;
        }
    }
    let ans = count;
    println!("{}", ans);
}
