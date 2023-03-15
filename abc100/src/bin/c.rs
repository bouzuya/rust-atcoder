use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let mut sum = 0_usize;
    for a_i in a {
        let mut x = a_i;
        let mut c = 0_usize;
        while x % 2 == 0 {
            x /= 2;
            c += 1;
        }
        sum += c;
    }
    let ans = sum;
    println!("{}", ans);
}
