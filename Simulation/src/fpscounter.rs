use std::collections::VecDeque;
use std::time::{Duration, Instant};

pub struct FpsCounter {
    t0: Instant,
    delta: VecDeque<Duration>,
    delta_real: VecDeque<Duration>,
}

impl FpsCounter {
    pub fn new(n_avg: usize) -> Self {
        FpsCounter {
            t0: Instant::now(),
            delta: VecDeque::from(vec![Duration::new(0, 0); n_avg]),
            delta_real: VecDeque::from(vec![Duration::new(0, 0); n_avg]),
        }
    }

    // call et the end of tick execution
    pub fn tick(&mut self) {
        let d = self.t0.elapsed(); // find delta
        self.t0 = Instant::now(); // update t0
        self.delta.pop_front();
        self.delta.push_back(d);
    }
    /*
        pub fn get_fps_simple(&self) -> u64 {
            let sum = self.delta.iter().map(|x| x.as_nanos()).sum::<u128>() as f64;
            let len = self.delta.len() as f64;
            (1000000000.0 / (sum / len)) as u64
        }
    */
    pub fn get_fps(&self) -> (u64, u64) {
        let sum = self.delta.iter().map(|x| x.as_nanos()).sum::<u128>() as f64;
        let len = self.delta.len() as f64;
        let current = (1000000000.0 / (sum / len)) as u64;

        let sum = self.delta_real.iter().map(|x| x.as_nanos()).sum::<u128>() as f64;
        let len = self.delta_real.len() as f64;
        let real = (1000000000.0 / (sum / len)) as u64;

        (current, real)
    }

    pub fn limit_fps(&mut self, limit: u32) {
        let d = self.t0.elapsed(); // find real delta
        self.delta_real.pop_front();
        self.delta_real.push_back(d);
        let target = Duration::from_nanos((1000000000.0 / (limit as f64)) as u64);
        if d >= target {
            return;
        }
        ::std::thread::sleep(target - d);
    }
}
