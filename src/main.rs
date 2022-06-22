#![feature(let_chains)]
use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use std::io::Write;

fn main() {
    if let Some(x) = std::env::args().nth(1) {
        if let Ok(x)=x.parse::<i32>() && x>=1 {rayon::ThreadPoolBuilder::new().num_threads(x as usize).build_global().unwrap();}
        let works: Vec<(usize, String)> = std::io::stdin()
            .lines()
            .enumerate()
            .map(|(i, x)| (i, x.unwrap()))
            .collect();
        std::fs::create_dir("dispatcher logs")
            .unwrap_or_else(|_| eprintln!("Warning: dispatcher logs folder exists."));
        works
            .par_iter()
            .progress_count(works.len() as u64)
            .for_each(dispatcher_body)
    } else {
        usage()
    }
}

fn dispatcher_body(idx:&(usize, String)) {
    let (idx,x)=idx;
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
    logger(format!("dispatcher logs/{idx}.out"), &out.stdout).unwrap_or_else(|x| {
        eprintln!("Warning: failed to write stdout for {} due to {}", idx, x)
    });
    logger(format!("dispatcher logs/{idx}.err"), &out.stderr).unwrap_or_else(|x| {
        eprintln!("Warning: failed to write stderr for {} due to {}", idx, x)
    });
}

fn logger(file: String, out: &[u8]) -> Result<(), std::io::Error> {
    let mut buffer = std::fs::File::create(file)?;
    buffer.write_all(out)?;
    Ok(())
}

fn usage() {
    println!("\x1b[0m\nUsage:\n  \x1b[1;32mcat\x1b[0m commands | \x1b[1;32m{0}\x1b[0m \x1b[1;33mnum_threads\x1b[0m",std::env::args().nth(0).unwrap())
}
