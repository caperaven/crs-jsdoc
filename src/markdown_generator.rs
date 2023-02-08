use crate::parameters;

// Enum to hold different markdown states
#[derive(Debug, PartialEq)]
enum MarkdownGeneratorState {
    None,

    // We are processing a comment block that will be part of the documentation
    Comment,

    // Processing parameters, @param
    Parameters,

    // Example block @example <caption>Example</caption>
    Example

}

#[derive(Debug, PartialEq)]
enum ExampleType {
    Js,
    Json,
    Html
}

pub struct MarkdownGenerator {
    current_state: MarkdownGeneratorState,

    // The lines of the markdown file
    lines: Vec<String>
}

impl MarkdownGenerator {
    pub fn new() -> MarkdownGenerator {
        MarkdownGenerator {
            current_state: MarkdownGeneratorState::None,
            lines: vec![],
        }
    }

    pub fn process_line(&mut self, line: String) {
        if line.trim().starts_with("/**") {
            self.current_state = MarkdownGeneratorState::Comment;
            self.lines.push(line.clone());
            return;
        }

        if line.trim().starts_with("*/") {
            self.current_state = MarkdownGeneratorState::None;
            self.lines.push(line.clone());
            return;
        }

        if self.current_state == MarkdownGeneratorState::Comment {
            self.lines.push(line.clone());
            return;
        }
    }

    pub fn generate(&mut self) -> String {
        let mut output: Vec<String> = vec![];

        for (_index, line) in self.lines.iter().enumerate() {
            let original_line = line.clone();
            let mut line = line.trim().replace("*", "").trim().to_string();

            if line.starts_with("/") {
                continue;
            }

            if line.starts_with("@class") {
                md_close_status(&mut output, &self.current_state);

                line = line.replace("@class", "").trim().to_string();
                md_headings(line.clone(), &mut output, "# class");
                continue;
            }

            if line.starts_with("@method") {
                md_close_status(&mut output, &self.current_state);

                line = line.replace("@method", "").trim().to_string();
                md_headings(line.clone(), &mut output, "## method");
                continue;
            }

            if line.starts_with("@function") {
                md_close_status(&mut output, &self.current_state);

                line = line.replace("@function", "").trim().to_string();
                md_headings(line.clone(), &mut output, "## function");
                continue;
            }

            if line.starts_with("@param") {
                md_close_status(&mut output, &self.current_state);

                line = line.replace("@param", "").trim().to_string();

                if self.current_state != MarkdownGeneratorState::Parameters {
                    output.push("\r".to_string());
                    output.push("***parameters***".to_string());
                    output.push("\r".to_string());
                    output.push("|parameter|type|description|required|default|".to_string());
                    output.push("|---------|----|-----------|--------|-------|".to_string());
                }

                self.current_state = MarkdownGeneratorState::Parameters;

                line = line.replace("@param", "").trim().to_string();
                md_parameters(line.clone(), &mut output);

                continue;
            }

            if line.starts_with("@example") {
                md_close_status(&mut output, &self.current_state);

                self.current_state = MarkdownGeneratorState::Example;
                line = line.replace("@example", "").trim().to_string();
                md_example(line.clone(), &mut output);
                continue;
            }

            if line.starts_with("@returns") {
                md_close_status(&mut output, &self.current_state);

                line = line.replace("@returns", "").trim().to_string();
                output.push(format!("**Returns**: {}", line));
                continue;
            }

            output.push(original_line.replace("*", ""));
        }

        return output.join("\r");
    }
}

fn md_close_status(output: &mut Vec<String>, status: &MarkdownGeneratorState) {
    match status {
        MarkdownGeneratorState::Example => {
            output.push("```".to_string());
        }
        _ => {
            return;
        }
    }
}

fn md_headings(line: String, output: &mut Vec<String>, heading_text: &str) {
    let parts = line.split("-").collect::<Vec<&str>>();

    let mut heading: Vec<&str> = vec![heading_text, "-"];
    heading.push(parts[0].trim());

    output.push(heading.join(" "));
}

fn md_parameters(line: String, output: &mut Vec<String>) {
    let param = parameters::Parameters::new(line.clone());

    output.push(format!("|{}|{}|{}|{}|{}|",
        param.param_name,
        param.param_type,
        param.param_description,
        param.param_required,
        param.param_default));
}

fn md_example(line: String, output: &mut Vec<String>) {
    let heading = line.replace("<caption>", "").replace("</caption>", "").trim().to_string();
    output.push(format!("***Example: {}***", heading));

    let mut example_type = ExampleType::Js;

    if heading.to_lowercase().contains("json") {
        example_type = ExampleType::Json;
    }

    if heading.to_lowercase().contains("html") {
        example_type = ExampleType::Html;
    }

    match example_type {
        ExampleType::Js => {
            output.push("```js".to_string());
        }
        ExampleType::Json => {
            output.push("```json".to_string());
        }
        ExampleType::Html => {
            output.push("```html".to_string());
        }
    }
}