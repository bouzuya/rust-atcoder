use proconio::input;

fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        a: [usize; n],
    };
    let mut count = 0;
    for i1 in 0..n {
        for i2 in i1 + 1..n {
            for i3 in i2 + 1..n {
                for i4 in i3 + 1..n {
                    for i5 in i4 + 1..n {
                        let mut x = a[i1];
                        x *= a[i2];
                        x %= p;
                        x *= a[i3];
                        x %= p;
                        x *= a[i4];
                        x %= p;
                        x *= a[i5];
                        x %= p;
                        if x == q {
                            count += 1;
                        }
                    }
                }
            }
        }
    }
    let ans = count;
    println!("{}", ans);
}
