use clap::Parser;
use std::env;
#[derive(Parser, Debug)]
#[clap(name = env!("CARGO_PKG_NAME"))]
#[clap(author = author())]
#[clap(version)]
#[clap(about = about(), long_about = None)]
pub struct Options {
    /// 'Removes audio sources'
    #[clap(long, short = 'a')]
    pub no_audio: bool,
    /// 'Sets custom base URL'
    #[clap(long, short = 'b', requires = "[http://localhost/]")]
    pub base_url: Option<String>,
    /// 'Removes CSS'
    #[clap(long, short = 'c')]
    pub no_css: bool,
    /// 'Enforces custom encoding'
    #[clap(long, short = 'c', requires = "[UTF-8]")]
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
    #[clap(long, short = 'o', requires = "[document.html]")]
    pub output: String,
    /// 'Suppresses verbosity'
    #[clap(long, short = 's')]
    pub silent: bool,
    /// 'Adjusts network request timeout'
    #[clap(long, short = 't', requires = "[60]",default_value_t = DEFAULT_NETWORK_TIMEOUT)]
    pub timeout: u64,
    /// 'Sets custom User-Agent string'
    #[clap(long, short = 'u', requires = "[Firefox]", default_value_t = DEFAULT_USER_AGENT.into())]
    pub user_agent: String,
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

impl Default for Options {
    fn default() -> Self {
        Self {
            no_audio: Default::default(),
            base_url: Default::default(),
            no_css: Default::default(),
            charset: Default::default(),
            ignore_errors: Default::default(),
            no_frames: Default::default(),
            no_fonts: Default::default(),
            no_images: Default::default(),
            isolate: Default::default(),
            no_js: Default::default(),
            insecure: Default::default(),
            no_metadata: Default::default(),
            output: Default::default(),
            silent: Default::default(),
            timeout: DEFAULT_NETWORK_TIMEOUT,
            user_agent: DEFAULT_USER_AGENT.into(),
            no_video: Default::default(),
            target: Default::default(),
            no_color: Default::default(),
            unwrap_noscript: Default::default(),
        }
    }
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
