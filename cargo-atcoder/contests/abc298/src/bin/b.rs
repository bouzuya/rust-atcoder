use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
        b: [[usize; n]; n],
    };

    let rotate = |a: &Vec<Vec<usize>>| -> Vec<Vec<usize>> {
        let mut c = vec![vec![0; n]; n];
        for i in 0..n {
            for j in 0..n {
                c[i][j] = a[n - 1 - j][i];
            }
        }
        c
    };

    let mut a = a;
    for _ in 0..4 {
        let mut ok = true;
        for i in 0..n {
            for j in 0..n {
                if a[i][j] == 1 && b[i][j] == 0 {
                    ok = false;
                }
            }
        }
        if ok {
            println!("Yes");
            return;
        }
        a = rotate(&a);
    }

    println!("No");
}
