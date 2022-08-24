#[test]
fn test_generate_from_url() {
    mod schema {
        schemafy::schemafy!("https://csrc.nist.gov/schema/nvd/feed/1.1/nvd_cve_feed_json_1.1.schema");
    }
}