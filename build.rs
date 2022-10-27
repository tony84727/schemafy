use schemafy_lib::Input;
use std::path::PathBuf;

fn main() {
    if cfg!(feature = "internal-regenerate") {
        let schema_path = "schemafy_lib/src/schema.json";
        schemafy_lib::Generator::builder()
            .with_root_name_str("Schema")
            .with_input(Input::File(PathBuf::from(schema_path)))
            .build()
            .generate_to_file("schemafy_lib/src/schema.rs")
            .unwrap();
    }
}
