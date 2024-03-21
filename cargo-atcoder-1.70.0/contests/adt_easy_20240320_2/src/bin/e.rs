use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        x: usize,
    };
    let sum_cycle = a.iter().copied().sum::<usize>();
    let count_cycle = x / sum_cycle;
    let mut count = count_cycle * n;
    let mut sum = count_cycle * sum_cycle;
    for a_i in a {
        sum += a_i;
        count += 1;
        if sum > x {
            break;
        }
    }
    let ans = count;
    println!("{}", ans);
}
