use self::mod_u32::ModU32;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    };
    let mut ans = ModU32::new(0);
    let mut cv = vec![ModU32::new(0); k + 1];
    for x in (1..=k).rev() {
        // gcd(a1, ..., an) = x となる数列 {Ai} がいくつあるか
        // gcd(a1, ..., an) が x の倍数になる個数から
        // 2x, 3x, ... を引くと gcd(a1, ..., an) = x が残る
        let mut count = ModU32::new((k / x) as u64).pow(n as u32);
        for i in 2.. {
            match cv.get(i * x) {
                Some(&c) => count -= c,
                None => break,
            }
        }
        cv[x] = count;
        ans += ModU32::new(x as u64) * count;
    }
    println!("{}", ans);
}

mod mod_u32 {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct ModU32(u32);

    const MODP: u32 = 1_000_000_007;

    impl ModU32 {
        pub fn inv(self) -> Self {
            if self.0 == 0 {
                panic!()
            };
            self.pow(MODP - 2)
        }

        pub fn new(x: u64) -> Self {
            Self((x % (MODP as u64)) as u32)
        }

        pub fn pow(self, exp: u32) -> Self {
            let mut b: ModU32 = self;
            let mut a: ModU32 = Self::new(1);
            let mut x = exp;
            while x > 1 {
                if x & 1 == 1 {
                    a *= b;
                }
                x /= 2;
                b *= b;
            }
            if x == 1 {
                a *= b;
            }
            a
        }
    }

    impl From<i32> for ModU32 {
        fn from(x: i32) -> Self {
            ModU32::from(x as i64)
        }
    }

    impl From<i64> for ModU32 {
        fn from(x: i64) -> Self {
            let n = x % (MODP as i64);
            ModU32::new(if n.is_negative() { MODP as i64 + n } else { n } as u64)
        }
    }

    impl From<u32> for ModU32 {
        fn from(x: u32) -> Self {
            ModU32::from(x as u64)
        }
    }

    impl From<u64> for ModU32 {
        fn from(x: u64) -> Self {
            ModU32::new(x)
        }
    }

    impl From<usize> for ModU32 {
        fn from(x: usize) -> Self {
            ModU32::from(x as u64)
        }
    }

    impl Into<i32> for ModU32 {
        fn into(self) -> i32 {
            self.0 as i32
        }
    }

    impl Into<i64> for ModU32 {
        fn into(self) -> i64 {
            self.0 as i64
        }
    }

    impl Into<u32> for ModU32 {
        fn into(self) -> u32 {
            self.0
        }
    }

    impl Into<u64> for ModU32 {
        fn into(self) -> u64 {
            self.0 as u64
        }
    }

    impl Into<usize> for ModU32 {
        fn into(self) -> usize {
            self.0 as usize
        }
    }

    impl std::fmt::Display for ModU32 {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl std::ops::Add<ModU32> for ModU32 {
        type Output = ModU32;

        fn add(self, rhs: ModU32) -> Self::Output {
            (self.0 as u64 + rhs.0 as u64).into()
        }
    }

    impl std::ops::AddAssign<ModU32> for ModU32 {
        fn add_assign(&mut self, rhs: ModU32) {
            self.0 = (*self + rhs).0
        }
    }

    impl std::ops::Sub<ModU32> for ModU32 {
        type Output = ModU32;

        fn sub(self, rhs: ModU32) -> Self::Output {
            (if self.0 < rhs.0 {
                self.0 + (MODP - rhs.0)
            } else {
                self.0 - rhs.0
            })
            .into()
        }
    }

    impl std::ops::SubAssign<ModU32> for ModU32 {
        fn sub_assign(&mut self, rhs: ModU32) {
            self.0 = (*self - rhs).0
        }
    }

    impl std::ops::Mul<ModU32> for ModU32 {
        type Output = ModU32;

        fn mul(self, rhs: ModU32) -> Self::Output {
            Self::new((self.0 as u64) * (rhs.0 as u64))
        }
    }

    impl std::ops::MulAssign<ModU32> for ModU32 {
        fn mul_assign(&mut self, rhs: ModU32) {
            self.0 = (*self * rhs).0
        }
    }

    impl std::ops::Div<ModU32> for ModU32 {
        type Output = ModU32;

        fn div(self, rhs: ModU32) -> Self::Output {
            self * rhs.inv()
        }
    }

    impl std::ops::DivAssign<ModU32> for ModU32 {
        fn div_assign(&mut self, rhs: ModU32) {
            self.0 = (*self / rhs).0
        }
    }
}
