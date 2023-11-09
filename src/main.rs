// use queue_file::QueueFile;
use clap::Parser;

#[derive(Parser)]
struct Args {
    /// The file to put queues
    file: String,
}

fn main() {
    let args = Args::parse();
    println!("{}", args.file)
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        // TODO
    }
}
