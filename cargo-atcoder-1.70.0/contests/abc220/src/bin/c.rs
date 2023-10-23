use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        x: usize,
    };

    let sum_a = a.iter().sum::<usize>();

    let count = x / sum_a * n;
    let x = x % sum_a;
    let mut sum = 0_usize;
    for (i, a_i) in a.iter().copied().enumerate() {
        sum += a_i;
        if sum > x {
            println!("{}", count + i + 1);
            return;
        }
    }
}
