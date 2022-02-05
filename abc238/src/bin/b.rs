use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let mut b = a
        .iter()
        .scan(0, |acc, &i| {
            *acc += i;
            *acc %= 360;
            Some(*acc)
        })
        .collect::<Vec<usize>>();
    b.push(0);
    b.push(360);
    b.sort();
    let mut max = 0;
    let mut p = 0;
    for b_i in b {
        max = max.max(b_i - p);
        p = b_i;
    }
    println!("{:?}", max);
}
