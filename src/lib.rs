pub struct Config {
    pub jwt_string: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let jwt_string = args[1].clone();

        Ok(Config { jwt_string })
    }
}