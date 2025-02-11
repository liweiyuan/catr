use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("wade")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .help("Number lines")
                .takes_value(false)
                .conflicts_with("number_nonblank"),
        )
        .arg(
            Arg::with_name("number_nonblank")
                .short("b")
                .long("number-nonblank")
                .help("Number non-blank lines")
                .takes_value(false),
        )
        .get_matches();
    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number_nonblank"),
    })
}

// 定义一个函数，名为 open，参数为文件名 filename，返回类型为自定义类型 MyResult，其中包含一个动态类型的 BufRead 接口的智能指针 Box
fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    // 使用 match 表达式来匹配文件名
    match filename {
        // 如果文件名是 "-"，表示要打开标准输入
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        // 如果文件名不是 "-"，则尝试打开文件
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(e) => eprintln!("Failed to open {}: {}", filename, e),
            Ok(file) => {
                let mut last_num = 0;
                for (line_num, line_result) in file.lines().enumerate() {
                    let line = line_result?;
                    if config.number_lines {
                        println!("{:6}\t{}", line_num + 1, line);
                    } else if config.number_nonblank_lines {
                        if !line.is_empty() {
                            last_num += 1;
                            println!("{:6}\t{}", last_num, line);
                        } else {
                            println!();
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(()) // 返回 Ok 值
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_standard_input() {
        let result = open("-");
        assert!(result.is_ok());
        //let reader = result.unwrap();
        //assert_eq!(&*reader, std::io::stdin());
    }

    #[test]
    fn test_open_file() {
        let filename = "test.txt";
        let result = open(filename);
        assert!(result.is_err());
        //let reader = result.unwrap();
        //let file = BufReader::new(std::fs::File::open(filename).unwrap());
        //assert_eq!(&*reader, &*file);
    }

    #[test]
    fn test_run_number_lines() {
        let config = Config {
            files: vec!["-".to_string()],
            number_lines: true,
            number_nonblank_lines: false,
        };
        let result = run(config);
        assert!(result.is_ok());
    }
}
