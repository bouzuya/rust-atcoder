use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
    };
    // 0 AB
    //   CD
    // 1 CA
    //   DB
    // 2 DC
    //   BA
    // 3 BD
    //   AC
    // 4 CD
    //   AB
    // 5 DB
    //   CA
    // 6 BA
    //   DC
    // 7 AC
    //   BD
    let mut dir = 0;
    let mut color = vec![vec![0; n]; n];
    let m = n - 1;
    let f = |r: usize, c: usize, dir: usize| {
        let rc = m - c;
        let rr = m - r;
        match dir {
            0 => (r, c),
            1 => (rc, r),
            2 => (rr, rc),
            3 => (c, rr),
            4 => (rr, c),
            5 => (rc, rr),
            6 => (r, rc),
            7 => (c, r),
            _ => unreachable!(),
        }
    };
    for _ in 0..q {
        input! { t: usize }
        match t {
            1 => {
                input! {
                    x: Usize1,
                    y: Usize1,
                }
                let (i, j) = f(x, y, dir);
                color[i][j] = (color[i][j] + 1) % 2;
            }
            2 => {
                input! { c: char }
                match c {
                    'A' => match dir {
                        0 | 1 | 2 | 3 => dir = (dir + 1) % 4,
                        4 | 5 | 6 | 7 => dir = (dir - 4 + 4 - 1) % 4 + 4,
                        _ => unreachable!(),
                    },
                    'B' => match dir {
                        0 | 1 | 2 | 3 => dir = (dir + 4 - 1) % 4,
                        4 | 5 | 6 | 7 => dir = (dir - 4 + 1) % 4 + 4,
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                }
            }
            3 => {
                input! { c: char }
                match c {
                    'A' => match dir {
                        0 | 1 | 2 | 3 => dir += 4,
                        4 | 5 | 6 | 7 => dir -= 4,
                        _ => unreachable!(),
                    },
                    'B' => match dir {
                        0 => dir = 6,
                        1 => dir = 7,
                        2 => dir = 4,
                        3 => dir = 5,
                        4 => dir = 2,
                        5 => dir = 3,
                        6 => dir = 0,
                        7 => dir = 1,
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }
    for x in 0..n {
        for y in 0..n {
            let (i, j) = f(x, y, dir);
            print!("{}", color[i][j]);
        }
        println!();
    }
}
