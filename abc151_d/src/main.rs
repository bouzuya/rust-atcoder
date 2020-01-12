fn read<T: std::str::FromStr>(
    stdin_lock: &mut std::io::StdinLock,
    buf: &mut Vec<u8>,
    delimiter: u8,
) -> T {
    buf.clear();
    let l = std::io::BufRead::read_until(stdin_lock, delimiter, buf).unwrap();
    buf.truncate(l - 1); // remove delimiter
    let s = unsafe { std::str::from_utf8_unchecked(&buf) };
    s.parse().unwrap_or_else(|_| panic!("read"))
}

fn main() {
    use std::cmp::min;

    let stdin = std::io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut buf: Vec<u8> = Vec::new();
    let h: usize = read(&mut stdin_lock, &mut buf, b' ');
    let w: usize = read(&mut stdin_lock, &mut buf, b'\n');
    let mut rv = vec![false; h * w];
    for y in 0..h {
        let s: String = read(&mut stdin_lock, &mut buf, b'\n');
        let lv: Vec<char> = s.chars().collect();
        for x in 0..w {
            rv[y * w + x] = lv[x] == '.';
        }
    }

    // for y in 0..h {
    //     for x in 0..w {
    //         print!("{}", if rv[y * w + x] { "." } else { "#" });
    //     }
    //     println!();
    // }

    let mut start = (0, 0);
    for i in 0..w * h {
        if rv[i] {
            start = (i / w, i % w);
            break;
        }
    }

    let mut cv: Vec<Option<usize>> = vec![None; h * w];
    let mut queue = Vec::new();
    queue.push((start, 0));
    while let Some(((x, y), c)) = queue.pop() {
        let cn = c + 1;

        // top
        if y > 0 {
            let xn = x;
            let yn = y - 1;
            let i = yn * w + xn;
            if rv[i] {
                if let Some(co) = cv[i] {
                    if cn < co {
                        cv[i] = Some(min(co, cn));
                        queue.push(((xn, yn), min(co, cn)));
                    }
                } else {
                    cv[i] = Some(cn);
                    queue.push(((xn, yn), cn));
                }
            }
        }

        // right
        if x + 1 < w {
            let xn = x + 1;
            let yn = y;
            let i = yn * w + xn;
            if rv[i] {
                if let Some(co) = cv[i] {
                    if cn < co {
                        cv[i] = Some(min(co, cn));
                        queue.push(((xn, yn), min(co, cn)));
                    }
                } else {
                    cv[i] = Some(cn);
                    queue.push(((xn, yn), cn));
                }
            }
        }

        // bottom
        if y + 1 < h {
            let xn = x;
            let yn = y + 1;
            let i = yn * w + xn;
            if rv[i] {
                if let Some(co) = cv[i] {
                    if cn < co {
                        cv[i] = Some(min(co, cn));
                        queue.push(((xn, yn), min(co, cn)));
                    }
                } else {
                    cv[i] = Some(cn);
                    queue.push(((xn, yn), cn));
                }
            }
        }

        // left
        if x > 0 {
            let xn = x - 1;
            let yn = y;
            let i = yn * w + xn;
            if rv[i] {
                if let Some(co) = cv[i] {
                    if cn < co {
                        cv[i] = Some(min(co, cn));
                        queue.push(((xn, yn), min(co, cn)));
                    }
                } else {
                    cv[i] = Some(cn);
                    queue.push(((xn, yn), cn));
                }
            }
        }
    }

    let mut mp = (0, (0, 0));
    for y in 0..h {
        for x in 0..w {
            let i = y * w + x;
            if let Some(v) = cv[i] {
                if v > mp.0 {
                    mp = (v, (x, y));
                }
            }
        }
    }

    cv = vec![None; h * w];
    queue.push((mp.1, 0));
    while let Some(((x, y), c)) = queue.pop() {
        let cn = c + 1;

        // top
        if y > 0 {
            let xn = x;
            let yn = y - 1;
            let i = yn * w + xn;
            if rv[i] {
                if let Some(co) = cv[i] {
                    if cn < co {
                        cv[i] = Some(min(co, cn));
                        queue.push(((xn, yn), min(co, cn)));
                    }
                } else {
                    cv[i] = Some(cn);
                    queue.push(((xn, yn), cn));
                }
            }
        }

        // right
        if x + 1 < w {
            let xn = x + 1;
            let yn = y;
            let i = yn * w + xn;
            if rv[i] {
                if let Some(co) = cv[i] {
                    if cn < co {
                        cv[i] = Some(min(co, cn));
                        queue.push(((xn, yn), min(co, cn)));
                    }
                } else {
                    cv[i] = Some(cn);
                    queue.push(((xn, yn), cn));
                }
            }
        }

        // bottom
        if y + 1 < h {
            let xn = x;
            let yn = y + 1;
            let i = yn * w + xn;
            if rv[i] {
                if let Some(co) = cv[i] {
                    if cn < co {
                        cv[i] = Some(min(co, cn));
                        queue.push(((xn, yn), min(co, cn)));
                    }
                } else {
                    cv[i] = Some(cn);
                    queue.push(((xn, yn), cn));
                }
            }
        }

        // left
        if x > 0 {
            let xn = x - 1;
            let yn = y;
            let i = yn * w + xn;
            if rv[i] {
                if let Some(co) = cv[i] {
                    if cn < co {
                        cv[i] = Some(min(co, cn));
                        queue.push(((xn, yn), min(co, cn)));
                    }
                } else {
                    cv[i] = Some(cn);
                    queue.push(((xn, yn), cn));
                }
            }
        }
    }

    let mut mp = (0, (0, 0));
    for y in 0..h {
        for x in 0..w {
            let i = y * w + x;
            if let Some(v) = cv[i] {
                if v > mp.0 {
                    mp = (v, (x, y));
                }
            }
        }
    }

    println!("{}", mp.0);
}
