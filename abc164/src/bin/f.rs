use proconio::input;

fn f(tbl: &mut Vec<Vec<Option<bool>>>, st: &Vec<Vec<u64>>, uv: &Vec<Vec<u64>>, i: usize) -> bool {
    let n = st[0].len();

    // (And && true) || (Or && false) の行を確定する
    for &flipped in [false, true].iter() {
        let ir = if flipped { 1 } else { 0 };
        for (y, (&ro, &rvbits)) in st[ir].iter().zip(uv[ir].iter()).enumerate() {
            let rv = (rvbits >> i) & 1;
            // (And (0) && true (1)) || (Or (1) && false (0))
            if ro != rv {
                let v = rv == 1;
                // その行はすべて rv
                for x in 0..n {
                    let (yt, xt) = if flipped { (x, y) } else { (y, x) };
                    match tbl[yt][xt] {
                        Some(b) if b != v => return false,
                        _ => tbl[yt][xt] = Some(v),
                    }
                }
            }
        }
    }

    // (And && false) || (Or && true) のうち確定できる行を確定する
    // 確定できる行: 未確定列が 1 列かつ、少なくとも 1 列は必要な列がない行
    for _ in 0..2 {
        for &flipped in [false, true].iter() {
            let ir = if flipped { 1 } else { 0 };
            for (y, (&ro, &rvbits)) in st[ir].iter().zip(uv[ir].iter()).enumerate() {
                let rv = (rvbits >> i) & 1;
                // (And (0) && false (0)) || (Or (1) && true (1))
                if ro == rv {
                    let v = rv == 1;
                    // その行は少なくとも 1 列の rv を含む
                    let mut ok = false;
                    let mut yxs = vec![];
                    for x in 0..n {
                        let (yt, xt) = if flipped { (x, y) } else { (y, x) };
                        match tbl[yt][xt] {
                            None => yxs.push((yt, xt)),
                            Some(b) if b == v => ok = true,
                            _ => {}
                        }
                    }
                    if !ok {
                        match yxs.len() {
                            0 => return false,
                            1 => {
                                let (yt, xt) = yxs[0];
                                tbl[yt][xt] = Some(v);
                            }
                            _ => {} // ignore
                        }
                    }
                }
            }
        }
    }

    // 残っているのは (And && false) || (Or && true) のうち
    // - 未確定 1 列以上の true / false どちらでも良い行 (ok 行なので無視する)
    // - 未確定 2 列以上の true / false どちらかでないといけない行
    // 2 列以上の場所については false / true を両方含めることで条件を満たせる
    let mut oks = vec![vec![false; n]; 2];
    for &flipped in [false, true].iter() {
        let ir = if flipped { 1 } else { 0 };
        for y in 0..n {
            let mut ok = true;
            for x in 0..n {
                let (yt, xt) = if flipped { (x, y) } else { (y, x) };
                if tbl[yt][xt].is_none() {
                    ok = false;
                    break;
                }
            }
            oks[ir][y] = ok;
        }
    }
    let mut cy = 0;
    for y in 0..n {
        let mut cx = 0;
        if oks[0][y] {
            continue;
        }
        for x in 0..n {
            if oks[1][x] {
                continue;
            }
            tbl[y][x] = Some((cy + cx) % 2 == 1);
            cx += 1;
        }
        cy += 1;
    }

    true
}

fn main() {
    input! {
        n: usize,
        st: [[u64; n]; 2],
        uv: [[u64; n]; 2]
    };
    let mut ans = vec![vec![0_u64; n]; n];
    for i in 0..64 {
        let mut tbl = vec![vec![None; n]; n];
        if !f(&mut tbl, &st, &uv, i) {
            println!("-1");
            return;
        }

        for y in 0..n {
            for x in 0..n {
                if tbl[y][x] == Some(true) {
                    ans[y][x] |= 1_u64 << i;
                }
            }
        }
    }
    for y in 0..n {
        for x in 0..n {
            print!("{}{}", ans[y][x], if x == n - 1 { "\n" } else { " " });
        }
    }
}
