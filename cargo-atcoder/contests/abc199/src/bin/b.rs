use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        b: [i64; n],
    };
    let mut count = 0;
    for x in 1..=1_000 {
        let mut ok = true;
        for (&a_i, &b_i) in a.iter().zip(b.iter()) {
            if !(a_i..=b_i).contains(&x) {
                ok = false;
                break;
            }
        }
        if ok {
            count += 1;
        }
    }
    let ans = count;
    println!("{}", ans);
}
