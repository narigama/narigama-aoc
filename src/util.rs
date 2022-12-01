use eyre::Context;
use reqwest::{cookie::Jar, Url};
use std::{path::PathBuf, str::FromStr, sync::Arc};

/// fetches (and caches) a given year/day of the AoC. Delete a file to
/// re-download it. Make sure your AOC_SESSION_ID in .env is set.
///
/// To get your AOC_SESSION_ID, check the application tab in chrome after
/// logging in. Look for the session=XYZ cookie and set AOC_SESSION_ID=XYZ
pub fn get_input(year: u64, day: u64) -> eyre::Result<String> {
    // grab the base url or use a default
    let base_url = std::env::var("AOC_URL").unwrap_or_else(|_| "https://adventofcode.com".into());

    eyre::ensure!(
        base_url.starts_with("http"),
        "AOC_URL (`{base_url}`) doesn't look like a url"
    );

    // validate year/day
    eyre::ensure!(
        (2015..=2022).contains(&year),
        "{year} is not a valid AoC year."
    );
    eyre::ensure!((1..=25).contains(&day), "{year} is not a valid AoC day.");

    // check for a cached file
    let base_dir_raw = std::env::var("AOC_INPUT_DIR").unwrap_or_else(|_| "input".into());
    let base_dir = PathBuf::from_str(&base_dir_raw)?;

    let file_path = base_dir
        .join(year.to_string())
        .join(format!("{day:0>2}.txt"));

    // cache hit! return the file
    if file_path.is_file() {
        tracing::debug!("cached input for {year}/{day:0>2} found!");
        return Ok(std::fs::read_to_string(file_path)?);
    }

    // cache miss! go get it
    tracing::debug!("{year}/{day:0>2} was not found, fetching...");

    // create a cookiejar
    let cookie_jar = Arc::new(Jar::default());
    cookie_jar.add_cookie_str(
        &format!("session={}", std::env::var("AOC_SESSION_ID")?),
        &Url::from_str(&base_url)?,
    );

    // build a client with the cookiejar
    let client = reqwest::blocking::ClientBuilder::default()
        .cookie_provider(cookie_jar)
        .build()?;

    // send a request
    let response = client
        .get(format!("{}/{year}/day/{day}/input", &base_url))
        .send()?
        // in the event of a non 200, check your AOC_SESSION_ID
        .error_for_status()
        .context("Check your AOC_SESSION_ID is valid")?
        // finally, grab the body
        .text()?
        .trim()
        .to_owned();

    // create the dirs and write the file
    std::fs::create_dir_all(
        file_path
            .parent()
            .ok_or_else(|| eyre::eyre!("couldn't create cache dirs"))?,
    )?;

    std::fs::write(file_path, &response)?;

    Ok(response)
}
