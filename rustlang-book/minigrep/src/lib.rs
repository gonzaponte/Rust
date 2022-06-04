use std::fs;
use std::env;
use std::error::Error;


#[derive(Debug)]
pub struct Config {
    pub query          : String,
    pub filename       : String,
    pub case_sensitive : bool,
    pub verbose        : bool,
}


impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next(); // ignore programme name

        let query          = match args.next() {
            Some(arg) => arg,
            None      => return Err("No query string provided"),
        };
        let filename       = match args.next() {
            Some(arg) => arg,
            None      => return Err("No filename provided"),
        };

        let case_sensitive = match env::var("CASE_INSENSITIVE") {
            Ok (value) => !vec!["1", "true"].contains(&value.as_str()),
            Err(_)     => true,
        };

        let verbose        = match args.next() {
            Some(arg) => arg.starts_with("-v"),
            None      => false,
        };

        Ok(Config { query, filename, case_sensitive, verbose })
    }
}


pub fn run(config : Config) -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string(config.filename)?;

    if config.verbose {
        println!("This is the text:");
        println!("{}", "=".repeat(50));
        for line in text.lines(){
            println!("{}", line);
        }
        println!("{}", "=".repeat(50));

        println!("");
        println!("These are the lines that match the query:");
    }

    let result = if config.case_sensitive { search                 (&config.query, &text) }
                 else                     { search_case_insensitive(&config.query, &text) };

    for line in result{
        println!("{}", line)
    }

    Ok(())
}


fn search<'a>(query : &str, contents : &'a str) -> Vec<&'a str> {
    let lines : Vec<&str> = contents.lines()
                                    .filter(|line| line.contains(query))
                                    .collect();
    lines
}


fn search_case_insensitive<'a>(query : &str, contents : &'a str) -> Vec<&'a str> {
    let lquery = query.to_lowercase();
    let lines : Vec<&str> = contents.lines()
                                    .filter(|line| line.to_lowercase().contains(&lquery))
                                    .collect();
    lines
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn config_attributes(){
        let query    = "abc123".to_string();
        let filename = "a_folder/a_filename.txt".to_string();
        let config   = Config { query          :    query.clone()
                              , filename       : filename.clone()
                              , case_sensitive : false
                              , verbose        : true
                              };

        assert_eq!( config.query   , query   );
        assert_eq!( config.filename, filename);
        assert!   (!config.case_sensitive    );
        assert!   ( config.verbose           );
    }

    // #[test]
    // fn config_new_ok(){
    //     let program  = "minigrep".to_string();
    //     let query    = "abc123".to_string();
    //     let filename = "a_folder/a_filename.txt".to_string();
    //     let args     = [program.clone(), query.clone(), filename.clone()];
    //     let config   = Config::new(&args).unwrap();
    //
    //     assert_eq!(config.query   , query   );
    //     assert_eq!(config.filename, filename);
    // }
    //
    // #[test]
    // fn config_new_err_too_few_args(){
    //     Config::new(&[                            ]).unwrap_err();
    //     Config::new(&[String::new()               ]).unwrap_err();
    //     Config::new(&[String::new(), String::new()]).unwrap_err();
    // }

    #[test]
    fn run_existing_file_does_not_panic(){
        let query          = String::new();
        let filename       = "/tmp/a_filename.txt".to_string();
        let case_sensitive = true;
        let verbose        = false;

        fs::write(&filename, &query).unwrap();
        let config   = Config { query, filename, case_sensitive, verbose };

        run(config).unwrap();
    }

    #[test]
    fn run_nonexisting_file(){
        let query    = "a123a".to_string();
        let filename = "a_filename.txt".to_string();

        let config   = Config { query          :    query.clone()
                              , filename       : filename.clone()
                              , case_sensitive : true
                              , verbose        : false
                              };

        run(config).unwrap_err();
    }

    #[test]
    fn search_single_line_case_sensitive(){
        let contents = "I am Groot".to_string();
        let tokens   = contents.split(" ");

        for query in tokens{
            let output = search(query, &contents);
            assert_eq!(output.len(), 1);
            assert_eq!(contents, *output.get(0).unwrap());
        }
    }

    #[test]
    fn search_multiline_single_value_case_sensitive(){
        let contents = "I am Groot\nHi there\nNice to meet you\n".to_string();
        let query    = "meet";
        let output   = search(query, &contents);
        let expected = contents.lines().nth(2).unwrap();

        assert_eq!(output.len(), 1);
        assert_eq!(expected, expected);
    }

    #[test]
    fn search_multiline_multivalue_case_sensitive(){
        let contents = "I am Alice\nI am Bob\nI am Claire".to_string();
        let query    = "am";
        let output   = search(query, &contents);
        let expected : Vec<&str> = contents.lines().collect();

        assert_eq!(output, expected);
    }

    #[test]
    fn search_single_line_case_insensitive(){
        let contents = "I am Groot".to_string();
        let tokens   = contents.split(" ");

        for query in tokens{
            let output = search_case_insensitive(&query.to_lowercase(), &contents);
            assert_eq!(output.len(), 1);
            assert_eq!(contents, *output.get(0).unwrap());

            let output = search_case_insensitive(&query.to_uppercase(), &contents);
            assert_eq!(output.len(), 1);
            assert_eq!(contents, *output.get(0).unwrap());
        }
    }

    #[test]
    fn search_multiline_single_value_case_insensitive(){
        let contents = "I am Groot\nHi there\nNice to meet you\n".to_string();
        let expected = contents.lines().nth(2).unwrap();

        let query0   = "nice";
        for i in 0..4 {
            let char   = query0.chars().nth(i).unwrap().to_string();
            let query  = query0.replace(char.as_str(), &char.as_str().to_ascii_lowercase());
            let output = search_case_insensitive(&query, &contents);
            assert_eq!(output.len(), 1);
            assert_eq!(expected, expected);
        }
    }

    #[test]
    fn search_multiline_multivalue_case_insensitive(){
        let contents = "I am Alice\nI am alIcE\nI am AliCe".to_string();
        let query    = "alice";
        let output   = search_case_insensitive(query, &contents);
        let expected : Vec<&str> = contents.lines().collect();

        assert_eq!(output, expected);
    }

}
