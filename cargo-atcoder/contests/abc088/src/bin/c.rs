use proconio::input;

fn main() {
    input! {
        c: [[usize; 3]; 3]
    };
    for a_1 in 0..=100 {
        if a_1 > c[0][0] {
            continue;
        }
        let b_1 = c[0][0] - a_1;
        if a_1 > c[0][1] {
            continue;
        }
        let b_2 = c[0][1] - a_1;
        if a_1 > c[0][2] {
            continue;
        }
        let b_3 = c[0][2] - a_1;

        for a_2 in 0..=100 {
            for a_3 in 0..=100 {
                let a = vec![a_1, a_2, a_3];
                let b = vec![b_1, b_2, b_3];
                let mut ok = true;
                for i in 0..3 {
                    for j in 0..3 {
                        if a[i] + b[j] != c[i][j] {
                            ok = false;
                        }
                    }
                }
                if ok {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}
