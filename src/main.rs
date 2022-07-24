use reqwest::blocking::ClientBuilder;
use url::Url;
use crawlerrust::LinkExtractor;
use crawlerrust::crawler::Crawler;
//use std::time::Duration;
use structopt::StructOpt;

/// web crawler
#[derive(StructOpt)]
struct Opt {
    /// Maximum number of pages to be crawled
    #[structopt(short="n")]
    naximum_pages: usize,
    /// URL where this program start crawling
    strat_page: Url,
}


fn main() -> eyre::Result<()> {
    env_logger::init();

    // 引数受け取り
    let opt = Opt::from_args();

    /* let url = std::env::args().nth(1)
        .unwrap_or("https://www.rust-lang.org".to_owned());

    let url = Url::parse(&url)?; */

    // クライアント生成
    let client = ClientBuilder::new().build()?;
    let extractor = LinkExtractor::from_client(client);

    let crawler = Crawler::new(&extractor, opt.strat_page);

    //let wait = Duration::from_millis(100);

    for url in crawler.take(opt.naximum_pages) {
        println!("{}", url);
        //std::thread::sleep(wait);
    }
    
    Ok(())
}
