use std::fs;

use anyhow::anyhow;

#[derive(thiserror::Error, Debug)]
pub enum MyError {
    #[error("this is just a simple error")]
    SimpleError,
    #[error("this is my error data {0} and {1}")]
    MyErrorVariant(String, u32),
    #[error("an error happened with stuff: {name} and {id}")]
    OtherErrorType {
        name: String,
        id: u32,
    }
}

fn main() -> anyhow::Result<()> {
    // cause_an_error()?;
    // parse_error("NaN")?;

    let my_error = MyError::MyErrorVariant(String::from("hey"), 32);
    println!("{}", my_error);

    let my_error = MyError::OtherErrorType {
        id: 30,
        name: String::from("Bob"),
    };
    println!("{}", my_error);

    let num = "NaN".parse::<usize>()?;

    println!("got {}", num);

    Ok(())
}

// fn main() {
//     println!("Hello, world!");

//     if let Err(err) = cause_an_error() {
//         println!("{{}}: {}", err); // just print context
//         println!("{{:#}}: {:#}", err); // prints context + underlying error
//         println!("{{:?}}: {:?}", err); // print backtrace
//         println!("{{:#?}}: {:#?}", err); // prints actual debug view
//     }

//     if let Err(err) = parse_error("NaN") {
//         println!("{{}}: {}", err); // just print context
//         println!("{{:#}}: {:#}", err); // prints context + underlying error
//         println!("{{:?}}: {:?}", err); // print backtrace
//         println!("{{:#?}}: {:#?}", err); // prints actual debug view
//     }
// }

fn cause_an_error() -> anyhow::Result<Vec<u8>> {
    // this coerces
    let stuff = fs::read("i-dont-exist.txt")?;

    Ok(stuff)
}

fn parse_error(num_str: &str) -> anyhow::Result<usize> {
    // num_str.parse::<usize>().with_context(|| format!("cannot parse {}", num_str))

    match num_str.parse::<usize>() {
        Err(error) => Err(anyhow!(error)),
        Ok(parsed) => Ok(parsed),
    }
}