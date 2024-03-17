use proconio::input;
use superslice::Ext;

fn f(cells: &mut Vec<Vec<bool>>, t: &Vec<(usize, usize)>, index: usize) -> bool {
    let h = cells.len();
    let w = cells[0].len();

    if index == t.len() {
        for i in 0..h {
            for j in 0..w {
                if !cells[i][j] {
                    return false;
                }
            }
        }
        return true;
    }

    for i in 0..h {
        for j in 0..w {
            if cells[i][j] {
                continue;
            }

            let (a, b) = t[index];
            if i + a > h || j + b > w {
                return false;
            }
            for k in i..i + a {
                for l in j..j + b {
                    if cells[k][l] {
                        return false;
                    }
                    cells[k][l] = true;
                }
            }

            return f(cells, t, index + 1);
        }
    }

    false
}

fn main() {
    input! {
        n: usize,
        h: usize,
        w: usize,
        ab: [(usize, usize); n],
    };
    for bits in 1..1 << n {
        let mut sum = 0_usize;
        let mut tiles = vec![];
        for i in 0..n {
            if (bits >> i) & 1 == 1 {
                tiles.push(ab[i]);
                sum += ab[i].0 * ab[i].1;
            }
        }
        if sum != h * w {
            continue;
        }

        let m = tiles.len();
        for dir in 0..1 << m {
            let mut t = vec![];
            for i in 0..m {
                let (a, b) = tiles[i];
                let (a, b) = if (dir >> i) & 1 == 1 { (a, b) } else { (b, a) };
                t.push((a, b));
            }

            t.sort();
            loop {
                let mut cells = vec![vec![false; w]; h];

                let (a, b) = t[0];
                if a <= h && b <= w {
                    for i in 0..a {
                        for j in 0..b {
                            cells[i][j] = true;
                        }
                    }
                    if f(&mut cells, &t, 1) {
                        println!("Yes");
                        return;
                    }
                }
                if !t.next_permutation() {
                    break;
                }
            }
        }
    }
    println!("No");
}
