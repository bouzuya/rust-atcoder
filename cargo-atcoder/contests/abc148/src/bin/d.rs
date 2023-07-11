use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let mut i = 1;
    for &a_i in a.iter() {
        if i == a_i {
            i += 1;
        }
    }
    let ans = if i == 1 { -1 } else { (n + 1 - i) as i64 };
    println!("{}", ans);
}
