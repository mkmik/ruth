use histogram::Histogram;
use std::time::{Duration, Instant};

fn work(silent: bool, name: String) -> Histogram {
    std::thread::sleep(Duration::from_secs(2));

    let mut h = Histogram::configure().max_value(100_000).build().unwrap();

    for _i in 0..100 {
        let start = Instant::now();

        let mut acc = 0;
        for _i in 0..2000000 {
            acc += 1;
        }

        let el = start.elapsed();
        h.increment(el.as_millis() as u64).unwrap();

        if !silent {
            eprintln!("{}: tick (computed {} in {:?}):\tPercentiles: p50: {}ms p90: {}ms p99: {}ms p999: {}ms", name, acc, el,
                h.percentile(50.0).unwrap(),
                h.percentile(90.0).unwrap(),
                h.percentile(99.0).unwrap(),
                h.percentile(99.9).unwrap(),
            );
        }
    }
    h
}

fn main() {
    for iwn in 1..7 {
        for bwn in 0..7 {
            let iw = (0..iwn)
                .map(|i| std::thread::spawn(move || work(true, format!("inter {}", i))))
                .collect::<Vec<_>>();

            let bw = (0..bwn)
                .map(|i| {
                    std::thread::spawn(move || {
                        assert_eq!(unsafe { libc::setpriority(0, 0, 10) }, 0);
                        work(true, format!("batch {}", i))
                    })
                })
                .collect::<Vec<_>>();

            let mut ih = Histogram::new();
            for j in iw {
                ih.merge(&j.join().unwrap());
            }

            let mut bh = Histogram::new();
            for j in bw {
                bh.merge(&j.join().unwrap());
            }

            let h = &ih;
            println!(
                "{},{},{},{},{},{}",
                iwn,
                bwn,
                h.percentile(50.0).unwrap(),
                h.percentile(90.0).unwrap(),
                h.percentile(99.0).unwrap(),
                h.percentile(99.9).unwrap(),
            );
        }
    }
}
