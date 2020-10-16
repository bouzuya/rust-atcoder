use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let mut i = 1_i64;
    for &a_i in a.iter() {
        if a_i == i {
            i += 1;
        }
    }
    if i == 1 {
        println!("{}", -1);
    } else {
        let ans = n as i64 - (i - 1);
        println!("{}", ans);
    }
}
