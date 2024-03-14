use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [i64; n],
    };
    let mut sum = 0_i64;
    let mut a = vec![];
    for s_i in s {
        a.push(s_i - sum);
        sum = s_i;
    }
    for a_i in a {
        println!("{}", a_i);
    }
}
