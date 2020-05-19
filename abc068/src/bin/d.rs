use proconio::input;

fn main() {
    input! {
        k: usize
    };
    let n = 50_usize;
    let mut a = vec![0_usize; n];
    let r = k / n;
    for i in 0..n {
        a[i] = i + r;
    }
    for i in 0..k % n {
        a[i] += n;
        for j in 0..n {
            if i != j {
                a[j] -= 1;
            }
        }
    }

    println!("{}", n);
    for i in 0..n {
        print!("{}{}", a[i], if i == n - 1 { "" } else { " " });
    }
}
