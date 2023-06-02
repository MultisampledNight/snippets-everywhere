mod ols;
mod ultisnips;

pub use ols::Ols;
pub use ultisnips::UltiSnips;

use crate::Backend;

pub fn all() -> Vec<Box<dyn Backend>> {
    vec![Box::new(Ols), Box::new(UltiSnips)]
}
