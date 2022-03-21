use clap::CommandFactory;

// replace this with literally anything better once you find out you how
#[path = "src/cli.rs"]
mod main;

fn main() -> std::io::Result<()> {
    let out_dir = std::path::PathBuf::from(
        std::env::var_os("OUT_DIR")
            .ok_or_else(|| std::io::ErrorKind::NotFound)?,
    );

    let cmd = main::Cli::command();

    let man = clap_mangen::Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    std::fs::write(out_dir.join("mybin.1"), buffer)?;

    Ok(())
}
