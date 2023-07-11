use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        x: usize,
        a: [usize; n],
    };
    let mut count = 0;
    for &a_i in a.iter() {
        count += 1 + (d - 1) / a_i;
    }
    let ans = count + x;
    println!("{}", ans);
}
