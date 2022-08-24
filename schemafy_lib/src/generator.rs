use crate::Expander;
use std::{
    io,
    path::{Path, PathBuf},
};

/// A configurable builder for generating Rust types from a JSON
/// schema.
///
/// The default options are usually fine. In that case, you can use
/// the [`generate()`](fn.generate.html) convenience method instead.
#[derive(Debug, PartialEq)]
#[must_use]
pub struct Generator<'a, 'b> {
    /// The name of the root type defined by the schema. If the schema
    /// does not define a root type (some schemas are simply a
    /// collection of definitions) then simply pass `None`.
    pub root_name: Option<String>,
    /// The module path to this crate. Some generated code may make
    /// use of types defined in this crate. Unless you have
    /// re-exported this crate or imported it under a different name,
    /// the default should be fine.
    pub schemafy_path: &'a str,
    /// The JSON schema file to read
    pub input_file: Input<'b>,
}

impl<'a, 'b> Generator<'a, 'b> {
    /// Get a builder for the Generator
    pub fn builder() -> GeneratorBuilder<'a, 'b> {
        GeneratorBuilder::default()
    }

    fn generate_from_file(&self, input: &'b Path) -> proc_macro2::TokenStream {
        let input_file = if input.is_relative() {
            let crate_root = get_crate_root().unwrap();
            crate_root.join(input)
        } else {
            PathBuf::from(input)
        };

        let json = std::fs::read_to_string(&input_file).unwrap_or_else(|err| {
            panic!("Unable to read `{}`: {}", input_file.to_string_lossy(), err)
        });

        let schema = serde_json::from_str(&json).unwrap_or_else(|err| {
            panic!(
                "Cannot parse `{}` as JSON: {}",
                input_file.to_string_lossy(),
                err
            )
        });
        let mut expander = Expander::new(self.root_name.as_deref(), self.schemafy_path, &schema);
        expander.expand(&schema)
    }

    fn generate_from_unknown(&self, input: &'b str) -> proc_macro2::TokenStream {
        unimplemented!()
    }

    pub fn generate(&self) -> proc_macro2::TokenStream {
        match self.input_file {
            Input::File(file) => self.generate_from_file(file),
            Input::Unknown(input) => self.generate_from_unknown(input),
        }
    }

    pub fn generate_to_file<P: ?Sized + AsRef<Path>>(&self, output_file: &'b P) -> io::Result<()> {
        use std::process::Command;
        let tokens = self.generate();
        let out = tokens.to_string();
        std::fs::write(output_file, &out)?;
        Command::new("rustfmt")
            .arg(output_file.as_ref().as_os_str())
            .output()?;
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
#[must_use]
pub struct GeneratorBuilder<'a, 'b> {
    inner: Generator<'a, 'b>,
}

impl<'a, 'b> Default for GeneratorBuilder<'a, 'b> {
    fn default() -> Self {
        Self {
            inner: Generator {
                root_name: None,
                schemafy_path: "::schemafy_core::",
                input_file: Input::File(Path::new("schema.json")),
            },
        }
    }
}

impl<'a, 'b> GeneratorBuilder<'a, 'b> {
    pub fn with_root_name(mut self, root_name: Option<String>) -> Self {
        self.inner.root_name = root_name;
        self
    }
    pub fn with_root_name_str(mut self, root_name: &str) -> Self {
        self.inner.root_name = Some(root_name.to_string());
        self
    }
    pub fn with_input_file<P: ?Sized + AsRef<Path>>(mut self, input_file: &'b P) -> Self {
        self.inner.input_file = Input::File(input_file.as_ref());
        self
    }
    pub fn with_input(mut self, input: &'b str) -> Self {
        self.inner.input_file = Input::Unknown(input);
        self
    }
    pub fn with_schemafy_path(mut self, schemafy_path: &'a str) -> Self {
        self.inner.schemafy_path = schemafy_path;
        self
    }
    pub fn build(self) -> Generator<'a, 'b> {
        self.inner
    }
}

fn get_crate_root() -> std::io::Result<PathBuf> {
    if let Ok(path) = std::env::var("CARGO_MANIFEST_DIR") {
        return Ok(PathBuf::from(path));
    }

    let current_dir = std::env::current_dir()?;

    for p in current_dir.ancestors() {
        if std::fs::read_dir(p)?
            .into_iter()
            .filter_map(Result::ok)
            .any(|p| p.file_name().eq("Cargo.toml"))
        {
            return Ok(PathBuf::from(p));
        }
    }

    Ok(current_dir)
}

#[derive(PartialEq, Debug)]
pub enum Input<'a> {
    File(&'a Path),
    Unknown(&'a str),
}
