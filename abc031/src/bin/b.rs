use proconio::input;

fn main() {
    input! {
        l: i64,
        h: i64,
        n: usize,
        a: [i64; n],
    };
    for &a_i in a.iter() {
        if a_i < l {
            println!("{}", l - a_i);
        } else if h < a_i {
            println!("-1");
        } else {
            println!("0");
        }
    }
}
