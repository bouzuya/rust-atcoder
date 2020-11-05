use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    };
    a.sort();

    let mut p = 0;
    let mut q = 0;
    for &a_i in a.iter() {
        if a_i >= 0 {
            p += 1;
        } else {
            q += 1;
        }
    }

    let q = if p == 0 {
        n - 1
    } else if q == 0 {
        1
    } else {
        q
    };
    let ans = a[q..].iter().sum::<i64>() - a[..q].iter().sum::<i64>();
    println!("{}", ans);
    let mut x1 = a[n - 1];
    for &y in a[1..q].iter() {
        println!("{} {}", x1, y);
        x1 -= y;
    }
    let mut x2 = a[0];
    for &y in a[q..n - 1].iter() {
        println!("{} {}", x2, y);
        x2 -= y;
    }
    println!("{} {}", x1, x2);
}
