use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;

use handlebars::Handlebars;

#[derive(Debug,Clone)]
pub struct UseCaseInput {
    pub package: String,
    pub name: String,
    pub fields: Vec<String>,
}

const UC_TEMPLATE_CONTENT: &str = include_str!("uc.hbs");

fn register_template() -> Result<Handlebars<'static>, handlebars::TemplateError> {
    let template_name = "uc";

    let mut handlebars = Handlebars::new();
    if handlebars.has_template(template_name) {
        println!("Template {} already registered", template_name);
        return Ok(handlebars);
    };
    handlebars.register_template_string(template_name, UC_TEMPLATE_CONTENT)?;
    Ok(handlebars)
}

#[derive(Debug)]
pub enum GenerateUseCaseError {
    RenderError,
    TemplateError,
    Error
}

pub fn generate(use_case_input: &UseCaseInput) -> Result<(File, String), GenerateUseCaseError> {
    let handlebars = register_template().map_err(|_| GenerateUseCaseError::TemplateError)?;
    let mut data = BTreeMap::new();
    data.insert("package_name".to_string(), use_case_input.package.clone());
    data.insert("class_name".to_string(), use_case_input.name.clone());
    let rendered = handlebars.render("uc", &data).map_err(|_| GenerateUseCaseError::RenderError)?;
    let file_name = format!("{}.java", use_case_input.name.clone());
    let mut file = File::create(&file_name).map_err(|_| GenerateUseCaseError::Error)?;
    file.write_all(rendered.as_bytes()).map_err(|_| GenerateUseCaseError::Error)?;
    Ok((file, file_name))
}
