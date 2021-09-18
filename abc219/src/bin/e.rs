use proconio::input;

fn bit(i: usize, j: usize) -> usize {
    1 << (15 - (i * 4 + j))
}

fn dfs(route: &mut usize, bits: usize, i: usize, j: usize) {
    if (*route & bit(i, j)) != 0 {
        return;
    }
    *route |= bit(i, j);

    let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
    for (dr, dc) in dir {
        let (nr, nc) = (i as i64 + dr, j as i64 + dc);
        if !(0..4 as i64).contains(&nr) || !(0..4 as i64).contains(&nc) {
            continue;
        }
        let (nr, nc) = (nr as usize, nc as usize);
        if (bits & bit(nr, nc)) == 0 {
            continue;
        }

        dfs(route, bits, nr, nc);
    }
}

fn dfs2(ok: &mut bool, used: &mut usize, bits: usize, i: usize, j: usize) {
    if *ok {
        return;
    }
    if (*used & bit(i, j)) != 0 {
        return;
    }
    *used |= bit(i, j);

    let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
    for (dr, dc) in dir {
        let (nr, nc) = (i as i64 + dr, j as i64 + dc);
        if !(0..4 as i64).contains(&nr) || !(0..4 as i64).contains(&nc) {
            *ok = true;
            return;
        }
        let (nr, nc) = (nr as usize, nc as usize);
        if (bits & bit(nr, nc)) != 0 {
            continue;
        }

        dfs2(ok, used, bits, nr, nc);
    }
}

fn main() {
    input! {
        a: [[usize; 4]; 4]
    };

    let mut mask = 0_usize;
    for i in 0..4 {
        for j in 0..4 {
            if a[i][j] == 1 {
                mask |= bit(i, j);
            }
        }
    }
    // println!("{:b}", mask);

    let mut count = 0;
    'loop_bits: for bits in 0..1_usize << 16 {
        if (bits & mask) != mask {
            continue;
        }

        let mut start = None;
        for i in 0..4 {
            for j in 0..4 {
                if (bits & bit(i, j)) != 0 {
                    start = Some((i, j));
                }
            }
        }
        match start {
            None => continue,
            Some((i, j)) => {
                let mut route = 0_usize;
                dfs(&mut route, bits, i, j);
                if route != bits {
                    continue;
                }
            }
        }

        for i in 0 + 1..4 - 1 {
            for j in 0 + 1..4 - 1 {
                if (bits & bit(i, j)) == 0 {
                    let mut ok = false;
                    let mut route = 0_usize;
                    dfs2(&mut ok, &mut route, bits, i, j);
                    if !ok {
                        continue 'loop_bits;
                    }
                }
            }
        }

        count += 1;
    }
    let ans = count;
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(bit(0, 0) & 0xFFFF, 0x8000);
        assert_eq!(bit(0, 1) & 0xFFFF, 0x4000);
        assert_eq!(bit(0, 2) & 0xFFFF, 0x2000);
        assert_eq!(bit(0, 3) & 0xFFFF, 0x1000);
        assert_eq!(bit(1, 0) & 0xFFFF, 0x0800);
        assert_eq!(bit(1, 1) & 0xFFFF, 0x0400);
        assert_eq!(bit(1, 2) & 0xFFFF, 0x0200);
        assert_eq!(bit(1, 3) & 0xFFFF, 0x0100);
        assert_eq!(bit(3, 0) & 0xFFFF, 0x0008);
        assert_eq!(bit(3, 1) & 0xFFFF, 0x0004);
        assert_eq!(bit(3, 2) & 0xFFFF, 0x0002);
        assert_eq!(bit(3, 3) & 0xFFFF, 0x0001);
    }
}
