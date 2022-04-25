use proconio::input;

fn main() {
    input! {
        a: [[usize; 3]; 3],
        n: usize,
        b: [usize; n],
    };
    let mut c = vec![vec![false; 3]; 3];
    for b_i in b {
        for i in 0..3 {
            for j in 0..3 {
                if a[i][j] == b_i {
                    c[i][j] = true;
                }
            }
        }
    }
    for i in 0..3 {
        let mut ok = true;
        for j in 0..3 {
            if !c[i][j] {
                ok = false;
            }
        }
        if ok {
            println!("Yes");
            return;
        }
    }
    for j in 0..3 {
        let mut ok = true;
        for i in 0..3 {
            if !c[i][j] {
                ok = false;
            }
        }
        if ok {
            println!("Yes");
            return;
        }
    }
    let mut ok = true;
    for i in 0..3 {
        if !c[i][i] {
            ok = false;
        }
    }
    if ok {
        println!("Yes");
        return;
    }
    let mut ok = true;
    for i in 0..3 {
        if !c[i][3 - 1 - i] {
            ok = false;
        }
    }
    if ok {
        println!("Yes");
        return;
    }

    println!("No");
}
