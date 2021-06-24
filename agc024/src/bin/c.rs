use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let mut count = 0;
    let mut a_p = 0;
    for (i, &a_i) in a.iter().enumerate() {
        if a_i > i || (a_i as i64 - a_p as i64) > 1 {
            println!("-1");
            return;
        }
        if a_i == a_p + 1 {
            count += 1;
        } else {
            count += a_i;
        }
        a_p = a_i;
    }
    let ans = count;
    println!("{}", ans);
}
