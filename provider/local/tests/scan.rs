#[test]
fn test_scan() {
    let scanner = rustic_local_provider::scanner::Scanner::new("assets");
    let res = scanner.scan().unwrap();

    assert_eq!(
        res,
        vec![rustic_local_provider::scanner::Track {
            path: "assets/bensound-ukulele.mp3".into(),
            title: "Ukulele".into(),
            artist: Some("Bensound".into()),
            album: None
        }]
    );
}
