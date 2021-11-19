use proconio::input;

fn main() {
    input! {
        n: usize,
        c: [[i64; n]; n],
    };

    let mut min_i = 0;
    for i in 0..n {
        if c[i][0] < c[min_i][0] {
            min_i = i;
        }
    }

    let mut a = vec![0; n];
    a[0] = c[0][0] - c[min_i][0];
    for i in 1..n {
        a[i] = a[i - 1] + c[i][0] - c[i - 1][0];
    }

    let mut b = vec![0; n];
    b[0] = c[0][0] - a[0];
    for j in 1..n {
        b[j] = b[j - 1] + c[0][j] - c[0][j - 1];
    }

    for i in 0..n {
        if a[i] < 0 {
            println!("No");
            return;
        }

        for j in 0..n {
            if b[j] < 0 {
                println!("No");
                return;
            }

            if a[i] + b[j] != c[i][j] {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
    for i in 0..n {
        print!("{}{}", a[i], if i < n { ' ' } else { '\n' });
    }
    for j in 0..n {
        print!("{}{}", b[j], if j < n { ' ' } else { '\n' });
    }
}
