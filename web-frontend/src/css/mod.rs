pub(crate) mod generated;

pub(crate) use generated::{C, M};
pub(crate) use tailwindcss_to_rust_macros::{ToOptionVecString, C, DC, M};

use std::fmt;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
#[builder(doc)]
pub(crate) struct Classes {
    #[builder(setter(into))]
    classes: String,
    #[builder(default = false)]
    with_standard_padding: bool,
}

impl fmt::Display for Classes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.with_standard_padding {
            write!(
                f,
                "{} {}",
                self.classes,
                "foo",
//                C![C.spc.px_2, C.spc.py_2, M![M.lg, C.spc.px_4]],
            )
        } else {
            write!(f, "{}", self.classes)
        }
    }
}
