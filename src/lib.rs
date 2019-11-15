#[macro_use]
extern crate failure;

mod endpoints;
pub mod models;

mod riot_api;

pub use self::endpoints::*;
pub use self::riot_api::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
