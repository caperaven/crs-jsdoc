pub struct Parameters {
    pub param_name: String,
    pub param_description: String,
    pub param_type: String,
    pub param_required: bool,
    pub param_default: String
}

impl Parameters {
    pub fn new(source: String) -> Parameters {
        let mut result = Parameters {
            param_name: "-".to_string(),
            param_description: "-".to_string(),
            param_type: "-".to_string(),
            param_required: true,
            param_default: "-".to_string()
        };

        param_from_string(&mut result, &source);

        result
    }
}

fn param_from_string(param: &mut Parameters, source: &String) {
    let mut source = param_type_from_string(param, source);
    source = param_default_from_string(param, &source);

    let parts = source.split("-").collect::<Vec<&str>>();

    // name is left side of the first dash
    param.param_name = parts[0].trim().to_string();

    // description is right side of the first dash
    if parts.len() > 1 {
        param.param_description = parts[1].trim().to_string();
    }
    else {
        param.param_description = "-".to_string();
    }
}

fn param_type_from_string(param: &mut Parameters, source: &String) -> String {
    param.param_type = extract_value(source, "{", "}").to_string();

    if source.contains("{") == false {
        param.param_type = "-".to_string();
        return source.clone();
    }

    let result = source.replace(&format!("{{{}}}", param.param_type), "").trim().to_string();
    result
}

fn param_default_from_string(param: &mut Parameters, source: &String) -> String {
    if let None = source.find("[") {
        return source.clone();
    }

    let value = extract_value(source, "[", "]").to_string();
    let parts = value.split("=").collect::<Vec<&str>>();

    param.param_name = parts[0].trim().to_string();
    param.param_default = parts[1].trim().to_string();
    param.param_required = false;

    let result = source.replace(&format!("[{}]", value), param.param_name.as_str()).trim().to_string();
    result
}

fn extract_value<'a>(input: &'a str, from_char: &'a str, to_char: &'a str) -> &'a str {
    if let Some(start) = input.find(from_char) {
        if let Some(end) = input.find(to_char) {
            return &input[start+1..end];
        }
    }

    ""
}
