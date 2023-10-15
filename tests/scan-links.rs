use std::collections::HashSet;

use brokr::Brokr;
use url::Url;

#[test]
fn test_scan_links() {
    let brokr = Brokr::default();

    let links = [
        Url::parse("https://link.random/there").unwrap(),
        Url::parse("https://random.link.here/da").unwrap(),
        Url::parse("https://google.com").unwrap(),
    ]
    .to_vec();

    let broken_links = brokr.find_broken_links(links, Some(10), vec![]);

    let broken_links = broken_links
        .iter()
        .map(|bl| bl.as_str())
        .collect::<HashSet<_>>();

    assert!(broken_links.contains("https://link.random/there"));
    assert!(broken_links.contains("https://random.link.here/da"));
    assert!(!broken_links.contains("https://google.com"));
}
