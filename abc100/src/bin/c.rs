use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let mut count = 0;
    for &a_i in a.iter() {
        let mut m = a_i;
        while m > 0 && m % 2 == 0 {
            m /= 2;
            count += 1;
        }
    }
    let ans = count;
    println!("{}", ans);
}
