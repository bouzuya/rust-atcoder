use proconio::input;

fn is_ok(c: &[Vec<i64>], a: &[i64], b: &[i64]) -> bool {
    for i in 0..3 {
        for j in 0..3 {
            if c[i][j] != a[i] + b[j] {
                return false;
            }
        }
    }
    true
}

fn main() {
    input! {
        c: [[i64; 3]; 3],
    }

    let mut a = vec![];
    for a_0 in 0..=100 {
        a.push(a_0);
        for a_1 in 0..=100 {
            a.push(a_1);
            for a_2 in 0..=100 {
                a.push(a_2);
                let mut b = vec![0; 3];
                for j in 0..3 {
                    b[j] = c[0][j] - a[0];
                }

                if is_ok(&c, &a, &b) {
                    println!("Yes");
                    return;
                }
                a.pop();
            }
            a.pop();
        }
        a.pop();
    }

    println!("No");
}
