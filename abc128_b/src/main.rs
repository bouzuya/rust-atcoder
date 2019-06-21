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

struct Restaurant {
    i: i32,
    s: String,
    p: i32,
}

fn main() {
    let stdin = std::io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut buf: Vec<u8> = Vec::new();
    let n: i32 = read(&mut stdin_lock, &mut buf, b'\n');
    let mut ts: Vec<Restaurant> = Vec::new();
    for i in 0..(n - 1) {
        ts.push(Restaurant {
            i: i + 1,
            s: read(&mut stdin_lock, &mut buf, b' '),
            p: read(&mut stdin_lock, &mut buf, b'\n'),
        });
    }
    ts.push(Restaurant {
        i: n,
        s: read(&mut stdin_lock, &mut buf, b' '),
        p: read(&mut stdin_lock, &mut buf, b'\n'),
    });

    ts.sort_by(|a, b| match a.s.cmp(&b.s) {
        Ordering::Less => Ordering::Less,
        Ordering::Greater => Ordering::Greater,
        Ordering::Equal => b.p.cmp(&a.p),
    });
    for t in ts.iter() {
        println!("{}", t.i);
    }
}
