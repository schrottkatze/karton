use clap::Parser;
use lazy_static::lazy_static;
use std::convert::Infallible;
use std::fmt;
use std::net::IpAddr;
use std::path::PathBuf;
use std::str::FromStr;

lazy_static! {
    pub static ref ARGS: Args = Args::parse();
}

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// The username for basic HTTP auth. 
    /// If unset, HTTP authentication stays disabled.
    ///
    /// WARNING: people opening pastas will have to authenticate too.
    #[clap(long, env = "MICROBIN_AUTH_USERNAME")]
    pub auth_username: Option<String>,

    /// Set a password for HTTP authentication. 
    /// If unset, HTTP authentication will not require a password.
    /// If `auth_username` is unset, this option will not have any effect.
    #[clap(long, env = "MICROBIN_AUTH_PASSWORD")]
    pub auth_password: Option<String>,

    /// Enable the option to make pastas editable.
    #[clap(long, env = "MICROBIN_EDITABLE")]
    pub editable: bool,

    /// The text displayed in the browser navigation bar.
    #[clap(long, env = "MICROBIN_TITLE")]
    pub title: Option<String>,

    /// The web interfaces' footer text.
    #[clap(long, env = "MICROBIN_FOOTER_TEXT")]
    pub footer_text: Option<String>,

    /// Hide the footer of the web interface.
    #[clap(long, env = "MICROBIN_HIDE_FOOTER")]
    pub hide_footer: bool,

    /// Hide the header of the web interface.
    #[clap(long, env = "MICROBIN_HIDE_HEADER")]
    pub hide_header: bool,

    /// Hide the logo in the header.
    #[clap(long, env = "MICROBIN_HIDE_LOGO")]
    pub hide_logo: bool,

    /// Disable the listing page.
    #[clap(long, env = "MICROBIN_NO_LISTING")]
    pub no_listing: bool,

    /// Enable syntax highlighting in pastas. 
    #[clap(long, env = "MICROBIN_HIGHLIGHTSYNTAX")]
    pub highlightsyntax: bool,

    /// The port to which to bind the server.
    #[clap(short, long, env = "MICROBIN_PORT", default_value_t = 8080)]
    pub port: u16,

    /// The IP adress to bind the server to.
    #[clap(short, long, env="MICROBIN_BIND", default_value_t = IpAddr::from([0, 0, 0, 0]))]
    pub bind: IpAddr,

    /// Enable the option to create private pastas.
    #[clap(long, env = "MICROBIN_PRIVATE")]
    pub private: bool,

    /// Disables most css, apart form some inline styles.
    #[clap(long, env = "MICROBIN_PURE_HTML")]
    pub pure_html: bool,

    /// The servers public path, making it possible to run microbin behind a reverse proxy subpath. 
    #[clap(long, env="MICROBIN_PUBLIC_PATH", default_value_t = PublicUrl(String::from("")))]
    pub public_path: PublicUrl,

    /// Enable creation of QR codes of pastas. Requires `public_path` to be set.
    #[clap(long, env = "MICROBIN_QR")]
    pub qr: bool,


    /// Disable adding/removing/editing pastas.
    #[clap(long, env = "MICROBIN_READONLY")]
    pub readonly: bool,

    /// The amount of worker threads that the server is allowed to have.
    #[clap(short, long, env = "MICROBIN_THREADS", default_value_t = 1)]
    pub threads: u8,

    /// Sets a time value for the garbage collector. Pastas that aren't accessed for the given
    /// amount of days will be deleted. Set to 0 to disable garbage collection.
    #[clap(short, long, env = "MICROBIN_GC_DAYS", default_value_t = 90)]
    pub gc_days: u16,

    /// Enable the option to delete after a given amount of reads.
    #[clap(long, env = "MICROBIN_ENABLE_BURN_AFTER")]
    pub enable_burn_after: bool,

    /// The default amount of reads for the self-delete mechanism.
    #[clap(short, long, env = "MICROBIN_DEFAULT_BURN_AFTER", default_value_t = 0)]
    pub default_burn_after: u16,

    /// Changes the UIs maximum width from 720 pixels to 1080.
    #[clap(long, env = "MICROBIN_WIDE")]
    pub wide: bool,

    /// Disable "Never" expiry setting.
    #[clap(long, env = "MICROBIN_NO_ETERNAL_PASTA")]
    pub no_eternal_pasta: bool,

    /// Set the default expiry time value.
    #[clap(long, env = "MICROBIN_DEFAULT_EXPIRY", default_value = "24hour")]
    pub default_expiry: String,

    /// Disable file uploading.
    #[clap(short, long, env = "MICROBIN_NO_FILE_UPLOAD")]
    pub no_file_upload: bool,

    // TODO: replace with simple path.
    /// Replace built-in CSS file with a CSS file provided by the linked URL.
    #[clap(long, env = "MICROBIN_CUSTOM_CSS")]
    pub custom_css: Option<String>,

    /// Replace built-in animal names file with custom names file for pasta links.
    /// The file must be newline seperated.
    #[clap(long, env = "MICROBIN_CUSTOM_NAMES")]
    pub custom_names: Option<PathBuf>,

    /// Enable the use of Hash IDs for shorter URLs instead of animal names.
    #[clap(long, env = "MICROBIN_HASH_IDS")]
    pub hash_ids: bool,
}

#[derive(Debug, Clone)]
pub struct PublicUrl(pub String);

impl fmt::Display for PublicUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for PublicUrl {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uri = s.strip_suffix('/').unwrap_or(s).to_owned();
        Ok(PublicUrl(uri))
    }
}
