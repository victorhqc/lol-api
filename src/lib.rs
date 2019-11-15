#[macro_use]
extern crate failure;

pub mod constants;
pub mod endpoints;
mod riot_api;

pub mod models;

pub use self::riot_api::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
