use proconio::input;

fn main() {
    input! {
        p: char,
        q: char,
    };
    let p = (p as u8 - b'A') as usize;
    let q = (q as u8 - b'A') as usize;
    let (p, q) = (p.min(q), p.max(q));

    let a = vec![0, 3, 1, 4, 1, 5, 9];
    let d = a
        .iter()
        .scan(0, |acc, &i| {
            *acc += i;
            Some(*acc)
        })
        .collect::<Vec<usize>>();
    let ans = d[q] - d[p];
    println!("{}", ans);
}
