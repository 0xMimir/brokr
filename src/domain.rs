use anyhow::Result;
use linkify::{LinkFinder, LinkKind};
use reqwest::blocking::get;
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

    pub fn find_links(&self, path: &String, extensions: &[&str]) -> Result<Vec<Url>> {
        let files = recurse_files(path, extensions)?;
        let links = files
            .into_iter()
            .filter_map(|path| self.extract_links_from_file(path).ok())
            .flatten()
            .collect();

        Ok(links)
    }

    pub fn extract_links_from_file(&self, path: PathBuf) -> Result<Vec<Url>> {
        let content = read_file(&path)?;
        self.extract_links_from_string(content)
    }

    pub fn extract_links_from_string(&self, content: String) -> Result<Vec<Url>> {
        let links = self.link_finder.links(&content);
        let urls = links
            .into_iter()
            .filter_map(|link| link.as_str().parse().ok())
            .collect();

        Ok(urls)
    }

    pub fn find_broken_links(&self, links: Vec<Url>, max_threads: Option<u8>) -> Vec<Url> {
        let max_threads = max_threads.unwrap_or(8);

        let running_threads = Arc::new(AtomicU8::new(0));
        let broken_links = Arc::new(Mutex::new(vec![]));

        for link in links {
            let running_threads = running_threads.clone();
            let broken_links = broken_links.clone();
            thread::spawn(move || {
                while running_threads.load(Ordering::Relaxed) > max_threads {
                    sleep(Duration::from_millis(1));
                }

                running_threads.fetch_add(1, Ordering::Relaxed);

                let is_broken = match get(link.as_str()) {
                    Ok(response) => response.error_for_status().is_err(),
                    Err(_) => true,
                };

                if is_broken {
                    if let Ok(mut lock) = broken_links.lock() {
                        lock.push(link);
                    }
                }

                running_threads.fetch_sub(1, Ordering::Relaxed);
            });
        }

        while running_threads.load(Ordering::Relaxed) > 0 {
            sleep(Duration::from_millis(1));
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
}
