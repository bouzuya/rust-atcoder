use proconio::{input, marker::Bytes};

fn popcount(n: usize) -> usize {
    n.count_ones() as usize
}

fn f(mut n: usize) -> usize {
    let mut count = 0_usize;
    while n != 0 {
        n = n % popcount(n);
        count += 1;
    }
    count
}

fn main() {
    input! {
        n: usize,
        x: Bytes,
    };
    let x = x
        .into_iter()
        .map(|b| (b - b'0') as usize)
        .collect::<Vec<usize>>();
    let pc_x = x.iter().filter(|&&x_i| x_i == 1).count();

    if pc_x == 0 {
        for _ in 0..n {
            println!("1");
        }
        return;
    }

    let sum = {
        let mut r0 = 0_usize;
        for x_i in x.iter().copied() {
            r0 <<= 1;
            r0 += x_i;
            r0 %= pc_x + 1;
        }
        let mut r1 = 0_usize;
        if pc_x > 1 {
            for x_i in x.iter().copied() {
                r1 <<= 1;
                r1 += x_i;
                r1 %= pc_x - 1;
            }
        }
        vec![r0, r1]
    };

    // pow[i][j] := 2.pow(j) % (pc_x + if i == 0 { 1 } else { -1 })
    let pow = {
        let mut pow = vec![vec![0; n]; 2];
        let mut p = 1_usize;
        for i in 0..n {
            pow[0][i] = p;
            p <<= 1;
            p %= pc_x + 1;
        }
        if pc_x > 1 {
            let mut p = 1_usize;
            for i in 0..n {
                pow[1][i] = p;
                p <<= 1;
                p %= pc_x - 1;
            }
        }
        pow
    };

    let mut ans = vec![0; n];
    for (i, x_i) in x.iter().copied().enumerate() {
        match x_i {
            0 => {
                let pc = pc_x + 1;
                let sum = (sum[x_i] + pow[x_i][n - 1 - i]) % pc;
                ans[i] = f(sum) + 1;
            }
            1 => {
                let pc = pc_x - 1;
                if pc == 0 {
                    ans[i] = 0;
                    continue;
                }
                let sum = (pc + sum[x_i] - pow[x_i][n - 1 - i]) % pc;
                ans[i] = f(sum) + 1;
            }
            _ => unreachable!(),
        }
    }
    for a in ans {
        println!("{}", a);
    }
}
