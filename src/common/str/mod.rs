pub fn uncapitalize(value: String) -> String {
    let mut uncapitalize_value = value.clone();
    uncapitalize_value.replace_range(0..1, &value[0..1].to_lowercase());
    uncapitalize_value
}