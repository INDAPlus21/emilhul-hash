mod hashtable;

use hashtable::{HashTable, hash::Hashable, data::Data};

use clap::Parser;
use anyhow::{Context, Result};

/// Search for a pattern in  a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    /*let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout.lock());

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file '{:?}'", args.path))?;

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }*/

    let mut table = HashTable::<[u8; 4], String>::new(4);
    let data = Data {key: 100_u32.to_be_bytes(), value: format!("Name: {}, Age: {}", "Test", 20) };
    table.insert(data)
        .with_context(|| format!("Could not insert data into table"))?;
    
    let data = Data {key: 101231312_u32.to_be_bytes(), value: format!("Name: {}, Age: {}", "Hash", 22) };
    table.insert(data)
        .with_context(|| format!("Could not insert data into table"))?;
    
    let data = Data {key: 99_u32.to_be_bytes(), value: format!("Name: {}, Age: {}", "Table", 80) };
    table.insert(data)
        .with_context(|| format!("Could not insert data into table"))?;

    table.print()
        .with_context(|| format!("Failed to print table"))?;
    
    table.delete(99_u32.to_be_bytes())
        .with_context(|| "Failed to remove data corresponding to key")?;
    
    table.print()
        .with_context(|| format!("Failed to print table"))?;

    Ok(())
}
