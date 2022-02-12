use cargo_snippet::snippet;
#[allow(clippy::module_inception, clippy::many_single_char_names)]
#[snippet]
/// The period is 2^128 - 1
pub mod xorshift {

    #[derive(Debug, Clone)]
    #[allow(dead_code)]
    pub struct Xorshift128 {
        x: u32,
        y: u32,
        z: u32,
        w: u32,
    }

    impl Default for Xorshift128 {
        fn default() -> Self {
            Xorshift128 {
                x: 123456789,
                y: 362436069,
                z: 521288629,
                w: 88675123,
            }
        }
    }
    impl Xorshift128 {
        pub fn new(seed: u32) -> Xorshift128 {
            let mut xorshift = Xorshift128::default();
            xorshift.z ^= seed;
            xorshift
        }

        pub fn gen(&mut self) -> u32 {
            let t = self.x ^ (self.x << 11);
            self.x = self.y;
            self.y = self.z;
            self.z = self.w;
            self.w = (self.w ^ (self.w >> 19)) ^ (t ^ (t >> 8));
            self.w
        }
    }
}
