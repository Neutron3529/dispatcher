#![feature(let_chains)]

use pbr::ProgressBar;
use std::io::Write;

fn main() {
    if let Some(x) = std::env::args().nth(1) {
        let num_threads;
        if let Ok(x)=x.parse::<i32>() && x>=1 {
            num_threads=x
        } else{
            num_threads=10
            }
        let mut works: Vec<(usize, String)> = std::io::stdin()
            .lines()
            .enumerate()
            .filter_map(|(i, x)| {
                if let Ok(x)=x && x.trim().len()>0 {Some((i, x))} else {None}
            })
            .collect();
        works.reverse();
        std::fs::create_dir("dispatcher logs")
            .unwrap_or_else(|_| eprintln!("Warning: dispatcher logs folder exists."));
        let pb = ProgressBar::new(works.len() as u64);
        let mutex = std::sync::Arc::new(std::sync::Mutex::new((works, pb)));
        let mut threads = Vec::with_capacity(num_threads as usize);
        for _ in 0..num_threads {
            let thread_mutex = mutex.clone();
            threads.push(std::thread::spawn(move || loop {
                let (id, x) = {
                    let mut a = thread_mutex.lock().unwrap();
                    a.1.inc();
                    if let Some(x) = a.0.pop() {
                        x
                    } else {
                        break;
                    }
                };
                dispatcher_body(id, &x)
            }))
        }
        threads.into_iter().for_each(|x| x.join().unwrap())
    } else {
        usage()
    }
}

fn dispatcher_body(idx: usize, x: &str) {
    let out = if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
            .arg("/C")
            .arg(x)
            .output()
            .expect("failed to execute process")
    } else {
        std::process::Command::new("sh")
            .arg("-c")
            .arg(x)
            .output()
            .expect("failed to execute process")
    };
    logger(format!("dispatcher logs/{idx}.out"), &out.stdout)
        .unwrap_or_else(|x| eprintln!("Warning: failed to write stdout for {} due to {}", idx, x));
    logger(format!("dispatcher logs/{idx}.err"), &out.stderr)
        .unwrap_or_else(|x| eprintln!("Warning: failed to write stderr for {} due to {}", idx, x));
}

fn logger(file: String, out: &[u8]) -> Result<(), std::io::Error> {
    let mut buffer = std::fs::File::create(file)?;
    buffer.write_all(out)?;
    Ok(())
}

fn usage() {
    println!("\x1b[0m\nUsage:\n  \x1b[1;32mcat\x1b[0m commands | \x1b[1;32m{0}\x1b[0m \x1b[1;33mnum_threads\x1b[0m",std::env::args().nth(0).unwrap())
}
