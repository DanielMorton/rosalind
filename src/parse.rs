use clap::{arg, ArgMatches, Command};

pub(crate) fn parse() -> ArgMatches {
    Command::new("magpie")
        .arg(arg!(--file <FILE>))
        .get_matches()
}

pub(crate) trait RosalindParse {
    fn get_file(&self) -> String;
    fn get_file_type(&self) -> String;
}

impl RosalindParse for ArgMatches {
    fn get_file(&self) -> String {
        match self.get_one::<String>("file") {
            Some(f) => f.to_owned(),
            None => panic!("No file name provided"),
        }
    }

    fn get_file_type(&self) -> String {
        match self
            .get_one::<String>("file")
            .and_then(|f| f.split('/').last())
            .and_then(|f| f.split("_").last())
            .and_then(|f| f.split(".").next())
        {
            Some(f) => f,
            None => panic!("No file name provided"),
        }
        .to_owned()
    }
}
