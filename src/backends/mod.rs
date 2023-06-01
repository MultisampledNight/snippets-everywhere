mod ols;
pub mod ultisnips;

pub use ols::Ols;

use crate::Backend;

pub fn all() -> Vec<Box<dyn Backend>> {
    vec![Box::new(Ols)]
}
