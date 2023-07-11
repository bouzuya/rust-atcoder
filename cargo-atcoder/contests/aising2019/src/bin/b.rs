use proconio::input;

fn main() {
    input! {
        n: usize,
        a: i64,
        b: i64,
        p: [i64; n],
    };
    let mut count = (0, 0, 0);
    for p_i in p {
        if p_i <= a {
            count.0 += 1;
        } else if a + 1 <= p_i && p_i <= b {
            count.1 += 1;
        } else if b + 1 <= p_i {
            count.2 += 1;
        } else {
            unreachable!();
        }
    }
    let &ans = vec![count.0, count.1, count.2].iter().min().unwrap();
    println!("{}", ans);
}
