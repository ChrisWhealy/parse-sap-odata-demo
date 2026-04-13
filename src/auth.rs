use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

const API_KEY_ENV: &str = "ODATA_API_KEY";

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

// ---------------------------------------------------------------------------------------------------------------------
// Fetch API Key from .env file
// ---------------------------------------------------------------------------------------------------------------------
pub fn fetch_auth() -> Result<String, String> {
    let mut api_key = String::from("unknown");

    // Try to obtain userid and password from environment variable file .env
    if let Ok(lines) = read_lines(".env") {
        for line in lines {
            match line {
                Ok(l) => {
                    if l.starts_with(API_KEY_ENV) {
                        let (_, u) = l.split_at(l.find("=").unwrap() + 1);
                        api_key = u.to_owned();
                    }
                }
                Err(_) => (),
            }
        }
    }

    if api_key.eq("unknown") {
        Err(format!("{API_KEY_ENV} missing from .env file"))
    } else {
        Ok(api_key)
    }
}
