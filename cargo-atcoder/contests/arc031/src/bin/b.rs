use proconio::input;
use proconio::marker::Chars;

fn f(a: &Vec<Vec<char>>) -> bool {
    let mut b = a.clone();
    let mut count = 0;
    let r = b.len();
    for i in 0..r {
        let c = b[i].len();
        for j in 0..c {
            if b[i][j] == 'x' {
                continue;
            }
            count += 1;
            // dfs
            let mut stack = vec![];
            stack.push((i, j));
            while let Some((y, x)) = stack.pop() {
                if b[y][x] == 'x' {
                    continue;
                }
                b[y][x] = 'x';
                let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
                for (d_r, d_c) in dir.iter() {
                    let (y_next, x_next) = (y as i64 + d_r, x as i64 + d_c);
                    if !(0..r as i64).contains(&y_next) || !(0..c as i64).contains(&x_next) {
                        continue;
                    }
                    let (y_next, x_next) = (y_next as usize, x_next as usize);
                    match b[y_next][x_next] {
                        'o' => stack.push((y_next, x_next)),
                        'x' => {}
                        _ => unreachable!(),
                    }
                }
            }
        }
    }
    count <= 1
}

fn main() {
    input! {
        mut a: [Chars; 10],
    };

    let mut is_ok = f(&a);
    for i in 0..10 {
        for j in 0..10 {
            if a[i][j] == 'o' {
                continue;
            }
            a[i][j] = 'o';
            is_ok |= f(&a);
            a[i][j] = 'x';
        }
    }
    let ans = is_ok;
    println!("{}", if ans { "YES" } else { "NO" });
}
