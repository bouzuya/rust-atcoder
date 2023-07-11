use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let mut sum = 0;
    for a_i in a {
        let mut m = a_i;
        while (m % 2 == 0) || (m % 3 == 2) {
            m -= 1;
            sum += 1;
        }
    }
    let ans = sum;
    println!("{}", ans);
}
