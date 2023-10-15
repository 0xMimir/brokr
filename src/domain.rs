use anyhow::Result;
use linkify::{LinkFinder, LinkKind};
use reqwest::{
    blocking::Client,
    header::{HeaderName, HeaderValue},
    redirect::Policy,
};
use std::{
    path::PathBuf,
    sync::{
        atomic::{AtomicU8, Ordering},
        Arc, Mutex,
    },
    thread::{self, sleep},
    time::Duration,
};
use url::Url;

use crate::files::{read_file, recurse_files};

pub struct Brokr {
    pub(crate) link_finder: LinkFinder,
}

impl Default for Brokr {
    fn default() -> Self {
        Self::new()
    }
}

impl Brokr {
    pub fn new() -> Self {
        let mut link_finder = LinkFinder::default();
        link_finder.url_must_have_scheme(true);
        link_finder.kinds(&[LinkKind::Url]);

        Self { link_finder }
    }

    pub fn find_links(
        &self,
        path: &String,
        extensions: &[String],
        filter_localhost: bool,
    ) -> Result<Vec<Url>> {
        let files = recurse_files(path, extensions)?;
        let links = files
            .into_iter()
            .filter_map(|path| self.extract_links_from_file(path, filter_localhost).ok())
            .flatten()
            .collect();

        Ok(links)
    }

    pub fn extract_links_from_file(
        &self,
        path: PathBuf,
        filter_localhost: bool,
    ) -> Result<Vec<Url>> {
        let content = read_file(&path)?;
        self.extract_links_from_string(content, filter_localhost)
    }

    pub fn extract_links_from_string(
        &self,
        content: String,
        filter_localhost: bool,
    ) -> Result<Vec<Url>> {
        let links = self.link_finder.links(&content);
        let urls = links
            .into_iter()
            .filter_map(|link| link.as_str().parse().ok())
            .filter(|url: &Url| {
                let host = url.host_str();

                !filter_localhost
                    || !(host == Some("localhost")
                        || host == Some("127.0.0.1")
                        || host == Some("0.0.0.0"))
            })
            .collect();

        Ok(urls)
    }

    pub fn find_broken_links(
        &self,
        links: Vec<Url>,
        max_threads: Option<u8>,
        allowed_statuses: Vec<u16>,
    ) -> Vec<Url> {
        let max_threads = max_threads.unwrap_or(8);

        let links = Arc::new(Mutex::new(links));
        let running_threads = Arc::new(AtomicU8::new(0));
        let broken_links = Arc::new(Mutex::new(vec![]));

        for _ in 0..max_threads {
            let running_threads = running_threads.clone();
            let broken_links = broken_links.clone();
            let links = links.clone();
            let allowed_statuses = allowed_statuses.clone();
            thread::spawn(move || {
                running_threads.fetch_add(1, Ordering::Relaxed);
                let client = Self::create_client();

                while let Some(link) = Self::take_last(&links) {
                    let is_broken = match client.get(link.as_str()).send() {
                        Ok(response) => {
                            let status = response.status().as_u16();

                            !(response.error_for_status().is_ok()
                                || allowed_statuses.contains(&status))
                        }
                        Err(_) => true,
                    };

                    if is_broken {
                        if let Ok(mut lock) = broken_links.lock() {
                            lock.push(link);
                        }
                    }
                }

                running_threads.fetch_sub(1, Ordering::Relaxed);
            });
        }

        while running_threads.load(Ordering::Relaxed) > 0 {
            sleep(Duration::from_millis(10));
        }

        let links = match broken_links.lock() {
            Ok(mut links) => {
                let mut broken_links = vec![];
                broken_links.extend(links.drain(..));
                broken_links
            }
            Err(_) => vec![],
        };

        links
    }

    fn take_last(links: &Arc<Mutex<Vec<Url>>>) -> Option<Url> {
        links.lock().ok()?.pop()
    }

    fn create_client() -> Client {
        Client::builder()
            .redirect(Policy::none())
            .default_headers(
                [(
                    HeaderName::from_static("user-agent"),
                    HeaderValue::from_static("brokr"),
                )]
                .into_iter()
                .collect(),
            )
            .build()
            .expect("Error creating client")
    }
}
