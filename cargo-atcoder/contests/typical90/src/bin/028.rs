use proconio::input;

fn main() {
    input! {
        n: usize,
        lxlyrxry: [(usize, usize, usize, usize); n],
    };
    let max_i = 1_000;
    let mut c = vec![vec![0_i64; max_i + 2]; max_i + 2];
    for &(lx, ly, rx, ry) in lxlyrxry.iter() {
        c[ly][lx] += 1;
        c[ly][rx] -= 1;
        c[ry][lx] -= 1;
        c[ry][rx] += 1;
    }
    for y in 0..max_i {
        for x in 0..max_i {
            c[y][x + 1] += c[y][x];
        }
    }
    for x in 0..max_i {
        for y in 0..max_i {
            c[y + 1][x] += c[y][x];
        }
    }

    let mut a = vec![0; max_i * max_i + 1];
    for y in 0..max_i {
        for x in 0..max_i {
            a[c[y][x] as usize] += 1;
        }
    }
    for i in 1..=n {
        println!("{}", a[i]);
    }
}
