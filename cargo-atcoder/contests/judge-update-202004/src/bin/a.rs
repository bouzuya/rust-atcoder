use proconio::input;

fn main() {
    input! {
        s: i64,
        l: i64,
        r: i64,
    };
    if (l..=r).contains(&s) {
        println!("{}", s);
    } else if s < l {
        println!("{}", l);
    } else if s > r {
        println!("{}", r);
    } else {
        unreachable!();
    }
}
