use proconio::{input, marker::Bytes};

fn f(n: usize) -> usize {
    if n == 0 {
        0
    } else {
        f(n % (n.count_ones() as usize)) + 1
    }
}

fn main() {
    input! {
        n: usize,
        x: Bytes,
    };
    let x = x
        .into_iter()
        .map(|x_i| (x_i - b'0') as usize)
        .collect::<Vec<usize>>();
    let pc_x = x.iter().filter(|&&x_i| x_i == 1).count();
    let mut ans = vec![0; n];
    for b in 0..=1 {
        let mut r_x = 0_usize;
        let pc = if b == 0 {
            pc_x + 1
        } else if pc_x > 1 {
            pc_x - 1
        } else {
            continue;
        };
        for &x_i in x.iter() {
            r_x *= 2;
            r_x %= pc;
            r_x += x_i;
        }
        let mut k = 1_usize;
        for (i, &x_i) in x.iter().enumerate().rev() {
            if x_i == b {
                let r = if b == 0 { r_x + k } else { r_x + pc - k } % pc;
                ans[i] = f(r) + 1;
            }
            k *= 2;
            k %= pc;
        }
    }

    for a in ans {
        println!("{}", a);
    }
}
