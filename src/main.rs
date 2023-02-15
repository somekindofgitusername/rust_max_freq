use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread;
use std::time::Duration;

fn main() {
    let mut max_freq = 0.0;

    loop {
        let file = File::open("/proc/cpuinfo").unwrap();
        let reader = BufReader::new(file);

        let mut new_max_freq = max_freq;

        for line in reader.lines() {
            let line = line.unwrap();
            if line.starts_with("cpu MHz") {
                let freq = line.split(":").nth(1).unwrap().trim().parse::<f32>().unwrap();
                if freq > new_max_freq {
                    new_max_freq = freq;
                }
            }
        }

        if new_max_freq > max_freq {
            max_freq = new_max_freq;
            println!("Maximum CPU frequency updated: {:.2} MHz", max_freq);
        }

        thread::sleep(Duration::from_millis(100));
    }
}
