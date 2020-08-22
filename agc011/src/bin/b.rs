use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    };
    a.sort();
    let s = a
        .iter()
        .scan(0, |acc, a_i| {
            *acc += a_i;
            Some(*acc)
        })
        .collect::<Vec<_>>();
    let mut c = 0;
    let mut a_p = a[0];
    for (i, &a_i) in a.iter().enumerate().skip(1) {
        if a_p * 2 >= a_i {
            a_p += a_i;
        } else {
            a_p = s[i];
            c = i;
        }
    }
    let ans = n - c;
    println!("{}", ans);
}
