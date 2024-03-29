use cargo_snippet::snippet;
#[allow(clippy::module_inception)]
#[snippet]
pub mod timer {
    use std::time::{Duration, Instant};
    #[derive(Default)]
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
            self.start.map(|start| Instant::now() - start)
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
