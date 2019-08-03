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
    let mut cv = vec![(0i32, 0i32, 0i32, 0i32); m];
    for i in 0..m {
        cv[i].0 = i as i32;
        cv[i].1 = read(&mut stdin_lock, &mut buf, b' ');
        cv[i].2 = read(&mut stdin_lock, &mut buf, b'\n');
    }

    cv.sort_by(|(_, p1, y1, _), (_, p2, y2, _)| match p1.cmp(p2) {
        Ordering::Less => Ordering::Less,
        Ordering::Greater => Ordering::Greater,
        Ordering::Equal => y1.cmp(y2),
    });
    let mut j = 1;
    cv[0].3 = 1;
    for i in 1..m {
        j = if cv[i - 1].1 == cv[i].1 { j + 1 } else { 1 };
        cv[i].3 = j;
    }
    cv.sort_by(|(i1, _, _, _), (i2, _, _, _)| i1.cmp(i2));

    for i in 0..m {
        println!("{:06}{:06}", cv[i].1, cv[i].3);
    }
}
