use cargo_snippet::snippet;
#[snippet]
pub mod timer {
    use std::time::{Duration, Instant};
    pub struct Timer {
        start: Option<Instant>,
    }
    impl Timer {
        pub fn new() -> Timer {
            Timer { start: None }
        }
        pub fn reset(&mut self) {
            self.start = None;
        }
        pub fn start(&mut self) {
            self.start = Some(Instant::now());
        }
        pub fn elapsed(&self) -> Option<Duration> {
            if let Some(start) = self.start {
                Some(Instant::now() - start)
            } else {
                None
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::timer::Timer;
    use std::time::Duration;
    #[test]
    fn test_timer() {
        let mut timer = Timer::new();
        timer.start();
        //sleep 200 millis
        std::thread::sleep(Duration::from_millis(200));
        let elapsed = timer.elapsed().unwrap();
        assert!(elapsed <= Duration::from_millis(210));

        timer.reset();
        assert_eq!(timer.elapsed(), None);
    }
}
