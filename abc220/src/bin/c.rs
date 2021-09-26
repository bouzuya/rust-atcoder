use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        x: usize,
    };
    let sum_cycle = a.iter().sum::<usize>();
    let mut count = n * (x / sum_cycle);
    let mut sum = sum_cycle * (x / sum_cycle);
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
