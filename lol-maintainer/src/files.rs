use handlebars::Handlebars;

use super::endpoints::ToDescriptor;

pub fn parse_file_template<T>(path: &'static str, data: T) -> String
where
    T: ToDescriptor,
{
    let mut reg = Handlebars::new();
    reg.register_template_file("file", path).unwrap();

    reg.render("file", &data.descriptor()).unwrap()
}
