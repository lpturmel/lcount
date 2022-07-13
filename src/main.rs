pub mod cli;
pub mod utils;

use anyhow::{bail, Result};
use clap::Parser;
use cli::Cli;
use std::fmt::Debug;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::time::Instant;

use self::utils::Lang;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let src_path = Path::new(&cli.src_path);

    if !src_path.exists() {
        bail!("The path '{}' does not exist", &cli.src_path);
    }

    let now = Instant::now();
    let entries_count = count_entries(&src_path, &cli.extension)?;

    println!("{} lines\n", entries_count);

    println!("Done in: {}ms", now.elapsed().as_millis());
    Ok(())
}

fn count_entries(dir: &Path, ext_opt: &Option<String>) -> anyhow::Result<usize> {
    let mut count = 0;
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                let entry_count = count_entries(&path, ext_opt)?;
                count += entry_count;
            } else {
                let file_extention =
                    path.extension()
                        .unwrap_or_default()
                        .to_str()
                        .ok_or_else(|| {
                            anyhow::anyhow!("The file '{}' has no extension", path.display())
                        })?;

                let file_language = Lang::from_str(file_extention);

                if let Some(_) = file_language {
                    match ext_opt {
                        Some(ext_opt) => {
                            let ext_opt_lang = Lang::from_str(ext_opt);

                            if let Some(_) = ext_opt_lang {
                                if ext_opt == file_extention {
                                    let line_count = count_lines(&path)?;
                                    count += line_count;
                                }
                            }
                        }
                        None => {
                            let line_count = count_lines(&path)?;
                            count += line_count;
                        }
                    }
                }
            }
        }
    } else {
        let line_count = count_lines(dir)?;
        count += line_count;
    }
    Ok(count)
}

fn count_lines<P>(path: P) -> Result<usize>
where
    P: AsRef<Path> + Debug,
{
    let file = File::open(&path)?;

    let line_count = std::io::BufReader::new(file).lines().count();
    Ok(line_count)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_can_count_lines_for_file() {
        let path = Path::new("./data/count.txt");
        let line_count = count_entries(&path, &None).unwrap();
        assert_eq!(line_count, 2);
    }

    #[test]
    fn it_can_count_lines_for_dir() {
        let path = Path::new("./data/dir");
        let line_count = count_entries(&path, &None).unwrap();
        assert_eq!(line_count, 8);
    }

    #[test]
    fn it_can_count_lines_for_dir_specific_files() {
        let path = Path::new("./data/dir");
        let line_count = count_entries(&path, &Some("rs".to_string())).unwrap();
        assert_eq!(line_count, 3);
    }
}
