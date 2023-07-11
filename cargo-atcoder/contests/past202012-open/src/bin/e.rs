use proconio::input;
use proconio::marker::Chars;

fn test(s: &Vec<Vec<char>>, t: &Vec<Vec<char>>) -> bool {
    let s_h = s.len();
    let s_w = s[0].len();
    let t_h = t.len();
    let t_w = t[0].len();
    for o_r in -(t_h as i64)..(s_h as i64) {
        for o_c in -(t_w as i64)..(s_w as i64) {
            let mut ok = true;
            for r in 0..t_h {
                if !ok {
                    break;
                }
                for c in 0..t_w {
                    match t[r][c] {
                        '.' => continue,
                        '#' => {
                            let (s_r, s_c) = (o_r + r as i64, o_c + c as i64);
                            if !(0..s_h as i64).contains(&s_r) {
                                ok = false;
                                continue;
                            }
                            if !(0..s_w as i64).contains(&s_c) {
                                ok = false;
                                continue;
                            }
                            let (s_r, s_c) = (s_r as usize, s_c as usize);
                            if s[s_r][s_c] == '#' {
                                ok = false;
                                continue;
                            }
                        }
                        _ => unreachable!("unknown char"),
                    }
                }
            }
            if ok {
                return true;
            }
        }
    }
    return false;
}

fn rotate(s: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let s_h = s.len();
    let s_w = s[0].len();
    let t_h = s_w;
    let t_w = s_h;
    let mut t = vec![vec![' '; t_w]; t_h];
    for r in 0..s_h {
        for c in 0..s_w {
            t[c][s_h - 1 - r] = s[r][c];
        }
    }
    t
}

// fn print_table(s: &Vec<Vec<char>>) {
//     let h = s.len();
//     let w = s[0].len();
//     for r in 0..h {
//         for c in 0..w {
//             print!("{}", s[r][c]);
//         }
//         println!();
//     }
//     println!();
// }

fn main() {
    input! {
        h: usize,
        _w: usize,
        s: [Chars; h],
        t: [Chars; h],
    };
    let mut x = t;
    for _ in 0..4 {
        if test(&s, &x) {
            println!("Yes");
            return;
        }
        x = rotate(&x);
    }
    println!("No");
}
