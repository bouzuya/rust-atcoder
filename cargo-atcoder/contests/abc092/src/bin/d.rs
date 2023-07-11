use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    let (h, w) = (100, 100);
    let mut black = vec![vec![false; w]; h];
    for i in 0..h / 2 {
        for j in 0..w {
            black[i][j] = true;
        }
    }

    let mut w_count = 1;
    let mut is_ok = a == w_count;
    for i in (0..h / 2).step_by(2) {
        if is_ok {
            break;
        }
        for j in (0..w).step_by(2) {
            black[i][j] = false;
            w_count += 1;
            is_ok = w_count >= a;
            if is_ok {
                break;
            }
        }
    }

    let mut b_count = 1;
    let mut is_ok = b == b_count;
    for i in (0..h / 2).step_by(2) {
        let i = h - 1 - i;
        if is_ok {
            break;
        }
        for j in (0..w).step_by(2) {
            black[i][j] = true;
            b_count += 1;
            is_ok = b_count >= b;
            if is_ok {
                break;
            }
        }
    }

    println!("{} {}", 100, 100);
    let mut table = String::new();
    for i in 0..h {
        for j in 0..w {
            table.push(if black[i][j] { '#' } else { '.' });
        }
        table.push('\n');
    }
    print!("{}", table);
}
