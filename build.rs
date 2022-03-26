use clap::CommandFactory;
use walkdir::WalkDir;
use sha2::{Sha256, Digest};

use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

// replace this with literally anything better once you find out you how
#[path = "src/cli.rs"]
mod main;

fn main() -> std::io::Result<()> {
    // Manpages

    let out_dir = std::path::PathBuf::from(
        std::env::var_os("OUT_DIR")
            .ok_or(std::io::ErrorKind::NotFound)?,
    );

    let cmd = main::Cli::command();

    let man = clap_mangen::Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    std::fs::write(out_dir.join("mybin.1"), buffer)?;

    // Hash version numbers

    let mut hasher = Sha256::new();

    let mut walker = WalkDir::new("src")
        .into_iter()
        .filter_entry(|e| !e.path().is_dir());
    while let Some(entry) = walker.next() {
        hasher.update(
            std::fs::read(entry.unwrap().path()).unwrap(),
        );
    }

    let src_hash = hasher.finalize();

    let final_hash = u64::from_str_radix(
        &format!(
            "{}{}",
            &include_str!(".git/refs/heads/main")[0..8],
            &src_hash[0..4]
                .iter()
                .map(|x| format!("{:x}", x))
                .collect::<String>()
        ),
        16,
    )
    .unwrap();

    let mut rng = ChaCha8Rng::seed_from_u64(final_hash);

    let pname = petname::Petnames::default().generate(&mut rng, 2, " ");

    println!(
        "cargo:rustc-env=HASHVER={}", pname,
    );

    Ok(())
}
