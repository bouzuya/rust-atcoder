use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let mut count = 0;
    for (i, &a_i) in a.iter().enumerate() {
        let i = i + 1;
        if (i % 2 != 0) && (a_i % 2 != 0) {
            count += 1;
        }
    }
    let ans = count;
    println!("{}", ans);
}
