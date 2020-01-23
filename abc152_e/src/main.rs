use std::ops;

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

const MODP: u64 = 1_000_000_007;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct ModU64(u64);

impl ModU64 {
    fn pow(self, mut x: u64) -> Self {
        let mut b = self;
        let mut a = ModU64(1);
        while x > 1 {
            if x & 1 == 1 {
                a = a * b;
            }
            x = x / 2;
            b = b * b;
        }
        if x == 1 {
            a = a * b;
        }
        a
    }
}

impl ops::Add<ModU64> for ModU64 {
    type Output = ModU64;

    fn add(self, rhs: ModU64) -> Self::Output {
        ModU64((self.0 + rhs.0) % MODP)
    }
}

impl ops::Mul<ModU64> for ModU64 {
    type Output = ModU64;

    fn mul(self, rhs: ModU64) -> Self::Output {
        ModU64(self.0 % MODP * rhs.0 % MODP)
    }
}

impl ops::Div<ModU64> for ModU64 {
    type Output = ModU64;

    fn div(self, rhs: ModU64) -> Self::Output {
        self * rhs.pow(MODP - 2)
    }
}

fn main() {
    use std::cmp::max;
    use std::collections::HashMap;

    let stdin = std::io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut buf: Vec<u8> = Vec::new();
    let n: usize = read(&mut stdin_lock, &mut buf, b'\n');
    let mut av = vec![0u64; n];
    for i in 0..n - 1 {
        av[i] = read(&mut stdin_lock, &mut buf, b' ');
    }
    av[n - 1] = read(&mut stdin_lock, &mut buf, b'\n');

    let mut pv = vec![0u64; 1_000_000 + 1];
    for i in 0..pv.len() {
        pv[i] = i as u64;
    }
    for i in 2..pv.len() {
        let mut j = i;
        while j < pv.len() {
            if pv[j] % i as u64 == 0 {
                pv[j] = i as u64;
            }
            j += i;
        }
    }

    let mut apv: Vec<HashMap<u64, u64>> = vec![HashMap::new(); n];
    for i in 0..av.len() {
        let mut j = av[i];
        while j != 1 {
            let p = pv[j as usize];
            *apv[i].entry(p).or_insert(0) += 1;
            j /= p;
        }
    }

    let mut l: HashMap<u64, u64> = HashMap::new();
    for a in apv.iter() {
        for (&k, &v) in a.iter() {
            if let Some(lv) = l.get_mut(&k) {
                *lv = max(*lv, v);
                continue;
            }
            l.insert(k, v);
        }
    }

    let mut lcm = ModU64(1);
    for (&k, &v) in l.iter() {
        for _ in 0..v {
            lcm = lcm * ModU64(k as u64);
        }
    }

    let mut ans = ModU64(0);
    for &a in av.iter() {
        let b = lcm / ModU64(a);
        ans = ans + b;
    }
    println!("{}", ans.0);
}
