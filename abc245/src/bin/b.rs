use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    for x in 0.. {
        if !a.contains(&x) {
            println!("{}", x);
            break;
        }
    }
}
