use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    src: String,

    #[arg(short, long)]
    dest: String,
}

pub struct Config {
    pub src: String,
    pub dest: String,
}

impl Config {
    pub fn new() -> Config {
        let args = Args::parse();

        return Self {
            src: args.src,
            dest: args.dest,
        };
    }
}
