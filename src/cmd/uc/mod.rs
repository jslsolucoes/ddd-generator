use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;

use handlebars::Handlebars;
use crate::common;

#[derive(Debug, Clone)]
pub struct GenerateUseCaseInput {
    pub package: String,
    pub name: String,
    pub fields: Vec<String>,
}

#[derive(Debug)]
pub struct GenerateUseCaseOutput {
    pub files: Vec<GenerateUseCaseFile>,
}

#[derive(Debug)]
pub struct GenerateUseCaseFile {
    pub file: File,
    pub file_name: String,
}

#[derive(Debug)]
pub enum GenerateUseCaseError {
    RenderError,
    TemplateError,
    Error,
}

const UC_TEMPLATE_NAME: &str = "uc";
const UC_TEMPLATE_CONTENT: &str = include_str!("uc.hbs");
const UC_CONTROLLER_TEMPLATE_NAME: &str = "uc_controller";
const UC_CONTROLLER_TEMPLATE_CONTENT: &str = include_str!("uc_controller.hbs");

fn register_template() -> Result<Handlebars<'static>, handlebars::TemplateError> {
    let templates = vec![
        (UC_TEMPLATE_NAME, UC_TEMPLATE_CONTENT),
        (UC_CONTROLLER_TEMPLATE_NAME, UC_CONTROLLER_TEMPLATE_CONTENT),
    ];

    let mut handlebars = Handlebars::new();

    for (template_name, template_content) in templates {
        if !handlebars.has_template(template_name) {
            handlebars.register_template_string(template_name, template_content)?;
        };
    }

    Ok(handlebars)
}

fn generate_uc(handlebars: &Handlebars, use_case_input: &GenerateUseCaseInput) -> Result<GenerateUseCaseFile, GenerateUseCaseError> {
    let package_name = use_case_input.package.clone();
    let class_name = use_case_input.name.clone();
    let class_name_field = common::str::uncapitalize(class_name.clone());

    let mut data = BTreeMap::new();
    data.insert("package_name".to_string(), package_name);
    data.insert("class_name".to_string(), class_name);
    data.insert("class_name_field".to_string(), class_name_field);

    let rendered = handlebars.render(UC_TEMPLATE_NAME, &data).map_err(|_| GenerateUseCaseError::RenderError)?;
    let file_name = format!("{}.java", use_case_input.name.clone());
    let mut file = File::create(&file_name).map_err(|_| GenerateUseCaseError::Error)?;
    file.write_all(rendered.as_bytes()).map_err(|_| GenerateUseCaseError::Error)?;
    Ok(GenerateUseCaseFile {
        file,
        file_name,
    })
}

fn generate_uc_controller(handlebars: &Handlebars, use_case_input: &GenerateUseCaseInput) -> Result<GenerateUseCaseFile, GenerateUseCaseError> {
    let package_name = use_case_input.package.clone();
    let class_name = use_case_input.name.clone();
    let class_name_field = common::str::uncapitalize(class_name.clone());

    let mut data = BTreeMap::new();
    data.insert("package_name".to_string(), package_name);
    data.insert("class_name".to_string(), class_name);
    data.insert("class_name_field".to_string(), class_name_field);

    let rendered = handlebars.render(UC_CONTROLLER_TEMPLATE_NAME, &data).map_err(|_| GenerateUseCaseError::RenderError)?;
    let file_name = format!("{}Controller.java", use_case_input.name.clone());
    let mut file = File::create(&file_name).map_err(|_| GenerateUseCaseError::Error)?;
    file.write_all(rendered.as_bytes()).map_err(|_| GenerateUseCaseError::Error)?;
    Ok(GenerateUseCaseFile {
        file,
        file_name,
    })
}

pub fn generate(use_case_input: &GenerateUseCaseInput) -> Result<GenerateUseCaseOutput, GenerateUseCaseError> {
    let handlebars = register_template().map_err(|_| GenerateUseCaseError::TemplateError)?;
    let uc = generate_uc(&handlebars, &use_case_input)?;
    let uc_controller = generate_uc_controller(&handlebars, &use_case_input)?;
    Ok(GenerateUseCaseOutput {
        files: vec![uc, uc_controller]
    })
}
