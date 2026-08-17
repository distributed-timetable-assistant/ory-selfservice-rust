#[test]
fn test_rewrite_kratos_redirect() {
    let kratos_url = "http://kratos-public:4433";
    let public_base = "http://localhost:3000";

    // Test replacement of kratos host with local host
    let input = "http://kratos-public:4433/self-service/login/browser?flow=123";
    let expected = "http://localhost:3000/self-service/login/browser?flow=123";
    let actual = input.replace(kratos_url, public_base);
    assert_eq!(actual, expected);
}
