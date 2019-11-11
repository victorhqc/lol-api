#[macro_use]
extern crate dotenv_codegen;

pub mod regions;
pub mod api;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
