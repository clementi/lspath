use clap::Parser;
use minus::Pager;
use std::env;
use std::error::Error;
use std::fmt::Write;

#[derive(Parser)]
#[command(author, version, about, long_about = Some("List the contents of the PATH environment variable"))]
struct Args {
    #[arg(short, long, default_value_t = false, help = "Page output")]
    page: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let key = "PATH";
    match env::var(key) {
        Ok(paths) => {
            let path_items = env::split_paths(&paths);
            if args.page {
                let mut output = Pager::new();
                for path in path_items {
                    writeln!(output, "{}", path.display())?;
                }
                minus::page_all(output)?;
            } else {
                for path in path_items {
                    println!("{}", path.display())
                }
            }
            Ok(())
        }
        Err(e) => {
            eprintln!("Could not get {}: {}", key, e);
            Err(Box::new(e))
        }
    }
}
