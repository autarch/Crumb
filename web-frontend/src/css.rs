use std::fmt;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
#[builder(doc)]
pub struct Classes {
    #[builder(setter(into))]
    classes: String,
    #[builder(default = false)]
    with_standard_padding: bool,
}

const STANDARD_CLASSES: &str = "px-2 py-2 lg:px-4";

impl fmt::Display for Classes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.with_standard_padding {
            write!(f, "{} {}", self.classes, STANDARD_CLASSES)
        } else {
            write!(f, "{}", self.classes)
        }
    }
}
