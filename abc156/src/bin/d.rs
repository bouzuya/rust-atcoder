use proconio::input;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ModI64(i64);

const MODP: i64 = 1_000_000_007;

impl ModI64 {
    pub fn inv(self) -> Self {
        self.pow(MODP as u32 - 2)
    }

    pub fn new(x: i64) -> Self {
        Self(x % MODP)
    }

    pub fn pow(self, exp: u32) -> Self {
        let mut b: ModI64 = self;
        let mut a: ModI64 = 1i64.into();
        let mut x = exp;
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

impl From<i64> for ModI64 {
    fn from(x: i64) -> Self {
        ModI64::new(x)
    }
}

impl std::fmt::Display for ModI64 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::ops::Add<ModI64> for ModI64 {
    type Output = ModI64;

    fn add(self, rhs: ModI64) -> Self::Output {
        (self.0 + rhs.0).into()
    }
}

impl std::ops::Sub<ModI64> for ModI64 {
    type Output = ModI64;

    fn sub(self, rhs: ModI64) -> Self::Output {
        let x = self.0 - rhs.0;
        (if x.is_negative() { MODP + x } else { x }).into()
    }
}

impl std::ops::Mul<ModI64> for ModI64 {
    type Output = ModI64;

    fn mul(self, rhs: ModI64) -> Self::Output {
        (self.0 * rhs.0).into()
    }
}

impl std::ops::Div<ModI64> for ModI64 {
    type Output = ModI64;

    fn div(self, rhs: ModI64) -> Self::Output {
        self * rhs.inv()
    }
}

fn comb(n: i64, r: i64) -> ModI64 {
    let mut a = ModI64::new(1);
    for i in (n - r + 1)..=n {
        a = a * ModI64::new(i);
    }
    let mut b = ModI64::new(1);
    for i in 1..=r {
        b = b * ModI64::new(i);
    }
    a / b
}

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    };

    if n == 2 {
        println!("{}", 0);
    } else {
        let u = ModI64::new(2).pow(n as u32) - ModI64::new(1);
        let ans = u - comb(n as i64, a as i64) - comb(n as i64, b as i64);
        println!("{}", ans);
    }
}
