use brokr::Brokr;

#[test]
fn test_domain() {
    let path = ".".to_owned();
    let extensions = vec!["md", "html", "txt"];
    let brokr = Brokr::default();
    let broken_links = brokr.find_broken_lines(&path, &extensions).unwrap();
    let links = broken_links
        .iter()
        .map(|bl| bl.url.as_str())
        .collect::<Vec<_>>();

    assert_eq!(
        links,
        [
            "https://somewhere.nowhere/there",
            "https://link.random/there",
            "https://random.link.here/da"
        ]
    )
}