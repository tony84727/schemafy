#[test]
fn test_generate_from_url() {
    schemafy_lib::Generator::builder()
        .with_root_name(None)
        .with_input("https://csrc.nist.gov/schema/nvd/feed/1.1/nvd_cve_feed_json_1.1.schema")
        .build()
        .generate();
}
