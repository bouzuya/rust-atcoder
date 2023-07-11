use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        x: usize,
        y: usize,
        a: [usize; n],
        b: [usize; m],
    };
    let mut i_a = 0;
    let mut i_b = 0;
    let mut c = 0;
    let mut t_a = 0;
    loop {
        let mut found = false;
        for i in i_a..n {
            i_a = i;
            if a[i] >= t_a {
                found = true;
                break;
            }
        }
        if !found {
            break;
        }
        let t_b = a[i_a] + x;
        c += 1;

        let mut found = false;
        for i in i_b..m {
            i_b = i;
            if b[i] >= t_b {
                found = true;
                break;
            }
        }
        if !found {
            break;
        }
        t_a = b[i_b] + y;
        c += 1;
    }

    let ans = c / 2;
    println!("{}", ans);
}
