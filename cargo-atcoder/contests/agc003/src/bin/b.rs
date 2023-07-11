use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut count = 0_usize;
    let mut prev = 0_usize;
    for a_i in a {
        if prev == 0 {
            count += a_i / 2;
            prev = a_i % 2;
        } else {
            count += (prev + a_i) / 2;
            prev = if a_i > 0 { a_i - 1 } else { 0 } % 2;
        }
    }
    let ans = count;
    println!("{}", ans);
}
