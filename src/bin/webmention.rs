#![feature(proc_macro_hygiene, decl_macro)]

extern crate async_std;
#[cfg(feature = "receive")]
#[macro_use]
extern crate rocket;
extern crate webmention;
extern crate url;
extern crate clap;
extern crate anyhow;

use anyhow::{Result, Context, anyhow};

use webmention::webmention::Webmention;
use webmention::wm_url::Url;

fn parse_url(u: &str) -> Result<Url> {
    let attempt = Url::parse(u);

    match attempt {
        Ok(url) => Ok(url),
        Err(url::ParseError::RelativeUrlWithoutBase) => {
            let with_http = "http://".to_owned() + u;
            return Url::parse(&with_http).with_context(|| format!("Failed to parse URL after prepending http:// prefix to <{}>", u));
        },
        Err(e) => Err(e.into())
    }
}

async fn send_link(input: (Url, Url)) -> Result<bool> {
    let (source_url, target_url) = input;

    println!(
        "{}\tsending webmention from {} to {}...",
        target_url, source_url, target_url
    );
    let mention = Webmention::from((&source_url, &target_url));
    webmention::sending::send_webmention(mention)
        .await
        .with_context(|| format!("Failed to send webmention from <{}> to <{}>", source_url, target_url))
}

async fn send_all(source: Url) -> Result<()> {
    println!("Fetching links from <{}>", source);
    let links = webmention::sending::fetch_links(&source).await
        .with_context(|| format!("Failed to fetch links from <{}>", source))?;
    if links.len() == 0 {
        println!("No links found");
    } else {
        println!("Links:");
        for (l, i) in links.iter().zip(1..(links.len())) {
            println!("{}:\t{}", i, l);
        }
    }

    let sending_vec: Vec<async_std::task::JoinHandle<_>> = links
        .into_iter()
        .zip(std::iter::repeat_with(|| source.clone()))
        .map(|l| async_std::task::spawn(async move { send_link((l.1, l.0)).await }))
        .collect();

    for handle in sending_vec.into_iter() {
        let result = handle.await;
        println!("{:?}", result);
    }

    Ok(())
}

#[cfg(feature = "receive")]
mod receive {
    use rocket::request::Form;
    use rocket::State;
    use webmention::storage::InMemoryWebmentionStorage;

    #[derive(FromForm)]
    struct WebmentionAttempt {
        source: String,
        target: String,
    }

    #[post("/webmention", data = "<webmention>")]
    fn webmention_endpoint(
        storage: State<InMemoryWebmentionStorage>,
        webmention: Form<WebmentionAttempt>,
    ) -> &'static str {
        let urls = (Url::parse(&webmention.source), Url::parse(&webmention.target));
        if let Ok(source_url) = urls.0 {
            if let Ok(target_url) = urls.1 {
                match async_std::task::block_on(webmention::receiving::receive_webmention(
                    &*storage,
                    &source_url,
                    &target_url,
                )) {
                    Ok(true) => return "OK",
                    Ok(false) => return "NOT OK",
                    Err(_) => return "ALSO NOT OK",
                }
            }
        }
        "NOT OK"
    }

    async fn start_receiver(_domain: Url) -> Result<()> {
        rocket::ignite()
            .manage(webmention::storage::InMemoryWebmentionStorage::new())
            .mount("/", routes![webmention_endpoint])
            .launch();
        Ok(())
    }
}

use clap::{App, Arg, SubCommand};

#[async_std::main]
async fn main() -> Result<()> {
    let app = App::new("webmention")
        .version("0.1.0")
        .author("Tim Marinin <mt@marinintim.com>")
        .about("Send and receive webmentions");

    let app = app.subcommand(
        SubCommand::with_name("send")
            .about("manually send webmentions")
            .arg(
                Arg::with_name("source")
                    .short("f")
                    .long("from")
                    .value_name("URL")
                    .help("The URL that we're linking from")
                    .takes_value(true)
                    .required(true)
            )
            .arg(
                Arg::with_name("target")
                    .short("t")
                    .long("to")
                    .value_name("URL")
                    .help("The URL that we had linked to")
            )
    );

    #[cfg(feature = "receive")]
    let app = app.subcommand(
        SubCommand::with_name("receive")
            .about("receive webmentions")
            .arg(
                Arg::with_name("domain")
                    .short("d")
                    .long("domain")
                    .value_name("URL")
                    .help("Domain for which we intend to receive webmentions")
                    .takes_value(true)
                    .required(true)
            )
    );

    let app = app.subcommand(
        SubCommand::with_name("discover-endpoint")
            .about("discover webmention endpoint")
            .arg(
                Arg::with_name("target")
                    .value_name("URL")
                    .help("URL that we want to discover endpoint for")
                    .index(1)
                    .required(true)
            )
    );


    let mut help = Vec::new();
    app.write_help(&mut help).expect("Could not write help");
    let help = String::from_utf8_lossy(&help);

    let matches = app.get_matches();

    if let Some(send_matches) = matches.subcommand_matches("send") {
        let source = send_matches.value_of("source").unwrap();
        let source = parse_url(source)
            .with_context(|| format!("Failed to parse source URL: <{}>", source))?;
        
        if let Some(target) = send_matches.value_of("target") {
            let target = parse_url(target)
                .with_context(|| format!("Failed to parse target URL: <{}>", target))?;
            
            send_link((source, target)).await?;
        } else {
            send_all(source).await?;
        }
        return Ok(());
    } else if let Some(_receive_matches) = matches.subcommand_matches("receive") {
        #[cfg(feature = "receive")]
        {
            let domain = _receive_matches.value_of("domain").unwrap();
            let domain = parse_url(domain).with_context(|| format!("Failed to parse domain URL: <{}>", domain))?;
            receiver::start_receiver(domain).await?;
            return Ok(());
        }
    } else if let Some(discover_matches) = matches.subcommand_matches("discover-endpoint") {
        let target = discover_matches.value_of("target").unwrap();
        let target = parse_url(target).with_context(|| format!("Failed to parse target URL: <{}>", target))?;
        let endpoint = webmention::endpoint_discovery::find_target_endpoint(&target).await?;
        if let Some(endpoint) = endpoint {
            println!("{}", endpoint);
        } else {
            println!("Endpoint of <{}> could not be determined", target);
        }
        return Ok(());
    }
    println!("{}", help);
    Err(anyhow!("No command specified"))
}

