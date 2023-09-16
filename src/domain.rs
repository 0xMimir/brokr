use anyhow::Result;
use linkify::{Link, LinkFinder, LinkKind, Links};
use reqwest::blocking::Client;
use std::{fmt::Display, path::PathBuf};
use url::Url;

use crate::files::{read_file, recurse_files};

pub struct Brokr {
    pub(crate) link_finder: LinkFinder,
    pub(crate) reqwest: Client,
}

impl Brokr {
    pub fn new() -> Self {
        let mut link_finder = LinkFinder::default();
        link_finder.url_must_have_scheme(true);
        link_finder.kinds(&[LinkKind::Url]);

        let reqwest = Client::default();
        Self {
            reqwest,
            link_finder,
        }
    }

    pub fn run(&self, path: &String, extensions: &Vec<&String>) -> Result<()> {
        let files = recurse_files(path, extensions)?;
        let mut _invalid_links = Vec::new();
        for path in files {
            if let Ok(mut invalid_links) = self.check_file(path) {
                _invalid_links.append(&mut invalid_links);
            };
        }

        if !_invalid_links.is_empty() {
            println!("\nFound {} invalid links\n", _invalid_links.len());
            for invalid_link in _invalid_links.iter() {
                println!("{}", invalid_link);
            }
        }

        Ok(())
    }

    pub(crate) fn check_file(&self, path: PathBuf) -> Result<Vec<InvalidLink>> {
        println!("Finding links in: {:?}", path);
        let content = read_file(&path)?;
        let links = self.link_finder.links(&content);
        self.check_links(links, &path)
    }

    pub(crate) fn check_links(&self, links: Links<'_>, file: &PathBuf) -> Result<Vec<InvalidLink>> {
        let mut invalid_links = Vec::new();
        for link in links {
            if let Err(link) = self.check_link(link, file) {
                invalid_links.push(link);
            }
        }
        Ok(invalid_links)
    }

    pub(crate) fn check_link(&self, link: Link, file: &PathBuf) -> Result<(), InvalidLink> {
        let url = match Url::parse(link.as_str()) {
            Ok(url) => url,
            Err(_) => return Ok(()), // Invalid url, this is parsing error
        };

        let check = self.get(url.clone());
        if !check {
            return Ok(());
        }

        let invalid_link = InvalidLink::new(url, file.to_owned());
        Err(invalid_link)
    }

    pub(crate) fn get(&self, url: Url) -> bool {
        let response = match self.reqwest.get(url).send() {
            Ok(valid) => valid,
            Err(_) => return true,
        };

        match response.error_for_status() {
            Ok(_) => false,
            Err(_) => true,
        }
    }
}

#[derive(Debug)]
pub struct InvalidLink {
    pub file: PathBuf,
    pub url: Url,
}

impl InvalidLink {
    pub fn new(url: Url, file: PathBuf) -> Self {
        Self { file, url }
    }
}

impl Display for InvalidLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "File: {:?}\nUrl: {}", self.file, self.url)
    }
}
