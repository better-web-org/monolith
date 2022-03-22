use clap::Parser;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::env;
#[derive(Parser, Debug, Default)]
#[clap(name = env!("CARGO_PKG_NAME"))]
#[clap(author = author())]
#[clap(version)]
#[clap(about = about(), long_about = None)]
pub struct Options {
    /// 'Removes audio sources'
    #[clap(long, short = 'a')]
    pub no_audio: bool,
    /// 'Sets custom base URL'
    #[clap(long, short = 'b', value_name = "http://localhost/")]
    pub base_url: Option<String>,
    /// 'Removes CSS'
    #[clap(long, short = 'c')]
    pub no_css: bool,
    /// 'Enforces custom encoding'
    #[clap(long, short = 'C', value_name = "UTF-8")]
    pub charset: Option<String>,
    /// 'Ignore network errors'
    #[clap(long, short = 'e')]
    pub ignore_errors: bool,
    /// 'Removes frames and iframes'
    #[clap(long, short = 'f')]
    pub no_frames: bool,
    /// 'Removes fonts'
    #[clap(long, short = 'F')]
    pub no_fonts: bool,
    /// 'Removes images'
    #[clap(long, short = 'i')]
    pub no_images: bool,
    /// 'Cuts off document from the Internet'
    #[clap(long, short = 'I')]
    pub isolate: bool,
    /// 'Removes JavaScript'
    #[clap(long, short = 'j')]
    pub no_js: bool,
    /// 'Allows invalid X.509 (TLS) certificates'
    #[clap(long, short = 'k')]
    pub insecure: bool,
    ///'Excludes timestamp and source information'
    #[clap(long, short = 'M')]
    pub no_metadata: bool,
    /// 'Writes output to <file>, use - for STDOUT'
    #[clap(long, short = 'o', value_name = "document.html")]
    pub output: Option<String>,
    /// 'Suppresses verbosity'
    #[clap(long, short = 's')]
    pub silent: bool,
    /// 'Adjusts network request timeout'
    #[clap(long, short = 't', value_name = "60",default_value_t = DEFAULT_NETWORK_TIMEOUT)]
    pub timeout: u64,
    /// 'Sets custom User-Agent string'
    #[clap(long, short = 'u', value_name = "Firefox",default_value_t = DEFAULT_USER_AGENT.into())]
    pub user_agent: String,
    /// Set the headers, for example:host:example.com
    ///
    /// Use `:` to separate name and value, and use spaces to separate each pair of name and value.
    #[clap(long, short = 'H', value_name = "name:value",parse(from_str=parse_headers_string))]
    pub headers: Option<HeaderMap>,
    ///'Removes video sources'
    #[clap(long, short = 'v')]
    pub no_video: bool,
    /// 'URL or file path, use - for STDIN'
    pub target: String,
    #[clap(skip = false)]
    pub no_color: bool,
    /// 'Replaces NOSCRIPT elements with their contents'
    #[clap(long, short = 'n')]
    pub unwrap_noscript: bool,
}
fn parse_headers_string(input: &str) -> HeaderMap {
    let mut res = HeaderMap::new();
    for (k, v) in input.split(' ').filter_map(|kv| kv.split_once(':')) {
        match HeaderName::from_bytes(k.as_bytes()) {
            Ok(name) => match HeaderValue::from_bytes(v.as_bytes()) {
                Ok(val) => {
                    if let Some(before) = res.insert(&name, val) {
                        eprintln!("{:?} before value: {:?}", name, before);
                    };
                }
                Err(e) => eprintln!("{:?}", e),
            },
            Err(e) => eprintln!("{:?}", e),
        };
    }
    res
}
const ASCII: &str = " \
 _____     ______________    __________      ___________________    ___
|     \\   /              \\  |          |    |                   |  |   |
|      \\_/       __       \\_|    __    |    |    ___     ___    |__|   |
|               |  |            |  |   |    |   |   |   |   |          |
|   |\\     /|   |__|    _       |__|   |____|   |   |   |   |    __    |
|   | \\___/ |          | \\                      |   |   |   |   |  |   |
|___|       |__________|  \\_____________________|   |___|   |___|  |___|
";
const DEFAULT_NETWORK_TIMEOUT: u64 = 120;
const DEFAULT_USER_AGENT: &str =
    "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:73.0) Gecko/20100101 Firefox/73.0";
const ENV_VAR_NO_COLOR: &str = "NO_COLOR";
const ENV_VAR_TERM: &str = "TERM";

impl Options {
    pub fn from_args() -> Options {
        let mut options = Options::parse();

        // Process the command
        options.no_color =
            env::var_os(ENV_VAR_NO_COLOR).is_some() || atty::isnt(atty::Stream::Stderr);
        if let Some(term) = env::var_os(ENV_VAR_TERM) {
            if term == "dumb" {
                options.no_color = true;
            }
        }

        options
    }
}

fn about() -> &'static str {
    clap::lazy_static::lazy_static! {
        static ref ABOUT_CACHED: String = format!("{}\n{}", ASCII, clap::crate_description!());
    }
    let s: &'static str = &*ABOUT_CACHED;
    s
}

fn author() -> &'static str {
    clap::lazy_static::lazy_static! {
        static ref AUTHOR_CACHED: String = format!("\n{}", env!("CARGO_PKG_AUTHORS").replace(':', "\n"));
    }
    let s: &'static str = &*AUTHOR_CACHED;
    s
}
