use clap::Parser;
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::{
    error::Error,
    fs::{DirEntry, File},
    io,
    path::PathBuf,
    sync::atomic::{AtomicUsize, Ordering},
    time::Instant,
};

use dashmap::DashMap;

mod pom;
use pom::{Pom, Repositories};

#[derive(Parser)]
struct Args {
    #[arg(default_value = "../java-repos/data/poms")]
    path: PathBuf,
}

fn main() {
    let args = Args::parse();

    let repos = DashMap::new();
    let distros = DashMap::new();
    let counter = AtomicUsize::new(0);

    let now = Instant::now();

    let errors: Vec<String> = args
        .path
        .read_dir()
        .unwrap()
        .par_bridge()
        .map(|el| process(&counter, &repos, &distros, el).map_err(|err| format!("{err:?}")))
        .filter_map(Result::err)
        .collect();

    let duration = Instant::now().duration_since(now);

    eprintln!("{} Errors Occured", errors.len());

    // TODO: Output results to csv

    println!("Found {} repos", repos.len());
    println!("Found {} distros", distros.len());
    println!(
        "Took {} seconds to parse {counter:?} POMs",
        duration.as_secs()
    );
}

fn process(
    ctr: &AtomicUsize,
    repos: &DashMap<String, usize>,
    distros: &DashMap<String, usize>,
    path: Result<DirEntry, io::Error>,
) -> Result<(), Box<dyn Error>> {
    let path = path.unwrap().path().join("pom.xml");
    let file = File::open(path)?;

    let pom: Pom = serde_xml_rs::from_reader(file)?;

    if let Some(Repositories { repositories }) = pom.repositories {
        for repo in repositories {
            repos.entry(repo.url).and_modify(|el| *el += 1).or_insert(1);
        }
    }

    if let Some(Repositories { repositories }) = pom.distribution_management {
        for repo in repositories {
            distros
                .entry(repo.url)
                .and_modify(|el| *el += 1)
                .or_insert(1);
        }
    }

    ctr.fetch_add(1, Ordering::Relaxed);

    Ok(())
}
