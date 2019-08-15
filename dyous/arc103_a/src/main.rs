// abc111_c (arc103_a)
use std::cmp::max;

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
    let stdin = std::io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut buf: Vec<u8> = Vec::new();
    let n: usize = read(&mut stdin_lock, &mut buf, b'\n');
    let mut vv = vec![0usize; n];
    for i in 0..n - 1 {
        vv[i] = read(&mut stdin_lock, &mut buf, b' ');
    }
    vv[n - 1] = read(&mut stdin_lock, &mut buf, b'\n');

    let mut me = vec![(0usize, 0i32); 100_000 + 1];
    let mut mo = vec![(0usize, 0i32); 100_000 + 1];
    for i in 0..n {
        if i % 2 == 0 {
            me[vv[i]].0 = vv[i];
            me[vv[i]].1 += 1;
        } else {
            mo[vv[i]].0 = vv[i];
            mo[vv[i]].1 += 1;
        }
    }

    me.sort_by(|&(_, v1), &(_, v2)| v2.cmp(&v1));
    mo.sort_by(|&(_, v1), &(_, v2)| v2.cmp(&v1));

    let ans = n as i32
        - if me[0].0 != mo[0].0 {
            me[0].1 + mo[0].1
        } else {
            max(me[0].1 + mo[1].1, me[1].1 + mo[0].1)
        };
    println!("{}", ans);
}
