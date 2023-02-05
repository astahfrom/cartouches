mod lib;

use std::env;
use std::fs;
use std::io::{self};
use std::path::Path;
use std::process::exit;

fn has_ext(p: &Path, ext: &str) -> bool {
    p.extension().map_or(false, |e| e == ext)
}

fn is_ascii_capitalized(s: &str) -> bool {
    s.chars().next().map_or(false, |c| c.is_ascii_uppercase())
}

fn snippets_path(path: &Path) -> io::Result<String> {
    if path.is_file() {
        let content = fs::read_to_string(path)?;
        Ok(lib::extract_snippets(content, String::new()))
    } else {
        let mut snippets: Vec<String> = vec![];

        for entry in path
            .read_dir()?
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| has_ext(&e.path(), "tex"))
            .filter(|e| e.file_name().to_str().map_or(false, is_ascii_capitalized))
        {
            let path = entry.path();
            let theory = path
                .file_stem()
                .expect("Could not get file stem.")
                .to_str()
                .expect("Could not convert to str.")
                .to_string();

            let content = fs::read_to_string(entry.path())?;
            let snips = lib::extract_snippets(content, theory);
            snippets.push(snips);
        }

        Ok(snippets.join("\n"))
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: ./{} tex-file/-dir snippets-out.tex", args[0]);
        exit(1);
    }

    let isa_path = Path::new(&args[1]);
    if !isa_path.exists() {
        println!(
            "The given output directory does not exist: {}",
            isa_path.display()
        );
        exit(1);
    }

    let snippets = snippets_path(&isa_path).expect("Could not extract snippets.");

    let snips_path = Path::new(&args[2]);
    fs::write(snips_path, snippets).expect("Could not write to snippets file.");

    println!("Snippets written to: {}", snips_path.display());
}
