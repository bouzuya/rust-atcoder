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

fn dfs(
    vv: &Vec<Vec<(usize, usize)>>,
    sv: usize,
    ev: usize,
    pv: usize,
    rv: &mut Vec<usize>,
) -> bool {
    if sv == ev {
        return true;
    }
    for &v in &vv[sv] {
        if v.0 == pv {
            continue;
        };
        if dfs(vv, v.0, ev, sv, rv) {
            rv.push(v.1);
            return true;
        }
    }
    return false;
}

fn main() {
    let stdin = std::io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut buf: Vec<u8> = Vec::new();

    // input
    let n: usize = read(&mut stdin_lock, &mut buf, b'\n');
    let mut abv = vec![(0usize, 0usize); n - 1];
    for i in 0..n - 1 {
        abv[i].0 = read(&mut stdin_lock, &mut buf, b' ');
        abv[i].1 = read(&mut stdin_lock, &mut buf, b'\n');
    }
    let m: usize = read(&mut stdin_lock, &mut buf, b'\n');
    let mut uvv = vec![(0usize, 0usize); m];
    for i in 0..m {
        uvv[i].0 = read(&mut stdin_lock, &mut buf, b' ');
        uvv[i].1 = read(&mut stdin_lock, &mut buf, b'\n');
    }

    let mut vv = vec![Vec::new(); n + 1];
    for i in 0..n - 1 {
        let (a, b) = abv[i];
        vv[a].push((b, i));
        vv[b].push((a, i));
    }

    let mut mv = vec![0u64; m];
    for i in 0..m {
        let (u, v) = uvv[i];
        let mut rv = Vec::new();
        if dfs(&vv, u, v, u, &mut rv) {
            for e in rv {
                mv[i] |= 1u64 << e;
            }
        }
    }

    let mut ans = 0u64;
    for i in 0u64..(1u64 << m) {
        let mut mask = 0u64;
        for j in 0..m {
            if (i >> j) & 1 == 1 {
                mask |= mv[j];
            }
        }
        let c = 1u64 << ((n - 1) as u32 - mask.count_ones());
        if i.count_ones() % 2 == 0 {
            ans += c;
        } else {
            ans -= c;
        };
    }
    println!("{}", ans);
}
