pub struct Config {
    pub src: String,
    pub dest: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let src = args[1].clone();
        let dest = args[2].clone();
        Ok(Config { src, dest })
    }
}
