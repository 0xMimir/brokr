use std::collections::HashSet;

use brokr::Brokr;

#[test]
fn test_scan_links() {
    let path = ".".to_owned();
    let extensions = vec!["md", "html", "txt"];
    let brokr = Brokr::default();
    let broken_links = brokr.find_links(&path, &extensions).unwrap();
    let links = broken_links
        .iter()
        .map(|bl| bl.as_str())
        .collect::<HashSet<_>>();

    assert!(links.contains("https://link.random/there"));
    assert!(links.contains("https://random.link.here/da"));
    assert!(links.contains("https://somewhere.nowhere/there"));
}
