use zed_extension_api as zed;

struct CucumberExtension;

impl zed::Extension for CucumberExtension {
    fn new() -> Self {
        CucumberExtension
    }
}

zed::register_extension!(CucumberExtension);
