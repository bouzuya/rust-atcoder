use proconio::input;

fn main() {
    input! {
        n: usize,
        mut ab: [(i64, i64); n],
    };
    ab.sort_by_key(|&(a, b)| (b, a));
    let mut c = 0;
    for (a_i, b_i) in ab {
        if c + a_i > b_i {
            println!("No");
            return;
        }
        c += a_i;
    }
    println!("Yes");
}
