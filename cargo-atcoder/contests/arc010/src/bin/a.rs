use proconio::input;

fn main() {
    input! {
        n: i64,
        m: usize,
        a: i64,
        b: i64,
        c: [i64; m]
    };

    let mut now = n;
    for (i, &c_i) in c.iter().enumerate() {
        if now <= a {
            now += b;
        }
        if now < c_i {
            println!("{}", i + 1);
            return;
        }
        now -= c_i;
    }
    println!("complete");
}
