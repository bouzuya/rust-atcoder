use std::cmp::Ordering;

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
    let _: usize = read(&mut stdin_lock, &mut buf, b' ');
    let m: usize = read(&mut stdin_lock, &mut buf, b'\n');
    let mut pyv = vec![(0i32, 0i32, 0i32); m];
    for i in 0..m {
        pyv[i].0 = i as i32;
        pyv[i].1 = read(&mut stdin_lock, &mut buf, b' ');
        pyv[i].2 = read(&mut stdin_lock, &mut buf, b'\n');
    }

    pyv.sort_by(|&(_, p1, y1), &(_, p2, y2)| match p1.cmp(&p2) {
        Ordering::Less => Ordering::Less,
        Ordering::Greater => Ordering::Greater,
        Ordering::Equal => y1.cmp(&y2),
    });

    let mut p = pyv[0].1;
    let mut c = 0;
    for i in 0..m {
        c = if p != pyv[i].1 { 1 } else { c + 1 };
        p = pyv[i].1;
        pyv[i].2 = c;
    }

    pyv.sort();
    for i in 0..m {
        println!("{:06}{:06}", pyv[i].1, pyv[i].2);
    }
}
