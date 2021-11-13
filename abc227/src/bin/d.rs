// WA
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n],
    };
    if n == 1 {
        println!("{}", a[0]);
        return;
    }
    if n == 2 {
        if k == 1 {
            println!("{}", a[0] + a[1]);
        } else {
            println!("{}", a[0].min(a[1]));
        }
        return;
    }

    a.sort();
    a.reverse();

    let mut r = k - 1;
    let mut l = k - 1;
    let mut count = a[r];

    while r < n - 1 {
        let mut moved = 0;
        while l > 0 && a[l] <= count {
            l -= 1;
            moved += 1;
        }
        r += moved;
        if moved == 0 {
            r += k;
        }
        if r < n {
            count += a[r];
        }
    }

    let ans = count;
    println!("{}", ans);
}
