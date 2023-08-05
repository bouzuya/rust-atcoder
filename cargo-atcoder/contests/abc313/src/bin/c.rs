use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    };
    let sum = a.iter().sum::<i64>();
    let avg = sum / n as i64;
    let num_max = sum - avg * n as i64;
    let num_min = n as i64 - num_max;
    let mut count_min = 0_i64;
    let mut sum = 0_i64;
    a.sort();
    for a_i in a {
        if count_min < num_min {
            sum += (avg - a_i).abs();
            count_min += 1;
        } else {
            sum += (avg + 1 - a_i).abs();
        }
    }
    let ans = sum / 2;
    println!("{}", ans);
}
