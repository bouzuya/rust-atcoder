use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let mut count = n;
    let mut prev = a[0];
    let mut len = 1;
    for a_i in a.into_iter().skip(1) {
        if prev < a_i {
            len += 1;
        } else {
            if len > 1 {
                count += len * (len - 1) / 2;
            }

            len = 1;
        }
        prev = a_i;
    }
    if len > 1 {
        count += len * (len - 1) / 2;
    }

    let ans = count;
    println!("{}", ans);
}
