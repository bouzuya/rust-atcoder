use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[i64; n]; 2],
    };
    let c1 = a[0]
        .iter()
        .scan(0, |acc, &x| {
            *acc += x;
            Some(*acc)
        })
        .collect::<Vec<i64>>();
    let mut c2 = a[1]
        .iter()
        .rev()
        .scan(0, |acc, &x| {
            *acc += x;
            Some(*acc)
        })
        .collect::<Vec<i64>>();
    c2.reverse();
    let ans = c1
        .iter()
        .zip(c2.iter())
        .map(|(&x, &y)| x + y)
        .max()
        .unwrap();
    println!("{}", ans);
}
