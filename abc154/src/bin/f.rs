use competition::{ModCombin, ModI64};
use proconio::input;

mod competition {
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

    pub struct ModCombin {
        factv: Vec<ModI64>,
        finvv: Vec<ModI64>,
    }

    impl ModCombin {
        pub fn new(max: usize) -> Self {
            let mut factv: Vec<ModI64> = vec![0.into(); max + 1];
            let mut finvv: Vec<ModI64> = vec![0.into(); max + 1];
            factv[0] = 1.into();
            for i in 1..=max {
                factv[i] = factv[i - 1] * (i as i64).into();
            }
            finvv[max] = factv[max].inv();
            for i in (1..=max).rev() {
                finvv[i - 1] = finvv[i] * (i as i64).into();
            }
            Self { factv, finvv }
        }

        pub fn combination(&self, n: usize, r: usize) -> ModI64 {
            if n < r {
                return 0.into();
            }
            self.factv[n] * self.finvv[r] * self.finvv[n - r]
        }
    }
}

fn g(mc: &competition::ModCombin, r: usize, c: usize) -> ModI64 {
    mc.combination((r + 1) + (c + 1), r + 1) - 1.into()
}

fn main() {
    input! {
        r1: usize,
        c1: usize,
        r2: usize,
        c2: usize
    };
    let mc = ModCombin::new(2_000_005);
    let ans = g(&mc, r2, c2) - g(&mc, r1 - 1, c2) - g(&mc, r2, c1 - 1) + g(&mc, r1 - 1, c1 - 1);
    println!("{}", ans);
}
