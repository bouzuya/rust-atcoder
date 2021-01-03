use proconio::input;

fn cumsum(a: &Vec<i64>) -> Vec<i64> {
    std::iter::once(0)
        .chain(a.iter().scan(0, |acc, &i| {
            *acc += i;
            Some(*acc)
        }))
        .collect()
}

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    };
    a.sort();
    let c = cumsum(&a);
    let sum = a
        .iter()
        .enumerate()
        .skip(1)
        .map(|(i, &a_i)| i as i64 * a_i - c[i])
        .sum::<i64>();
    let ans = sum;
    println!("{}", ans);
}
