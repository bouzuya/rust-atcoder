use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    };
    a.sort();
    let sum = a.iter().sum::<usize>();
    let avg = sum / n;
    let avg2 = sum - avg * n;
    let ans = vec![avg; n - avg2]
        .into_iter()
        .chain(vec![avg + 1; avg2])
        .zip(a)
        .map(|(x, y)| (x as i64 - y as i64).unsigned_abs() as usize)
        .sum::<usize>()
        / 2;
    println!("{}", ans);
}
