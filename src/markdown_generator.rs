// Enum to hold different markdown states
#[derive(Debug, PartialEq)]
enum MarkdownGeneratorState {
    None,

    // We are processing a comment block that will be part of the documentation
    Comment,

    // Processing the @function block or @class block or @method block
    Heading,

    // Processing parameters, @param
    Parameters,

    // Example block @example <caption>Example</caption>
    Example

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
        self.current_state = MarkdownGeneratorState::None;

        let mut output: Vec<String> = vec![];

        for (index, line) in self.lines.iter().enumerate() {
            let mut line = line.trim().replace("*", "").trim().to_string();

            if line.starts_with("/") {
                continue;
            }

            println!("{}: {}", index, line);

            if line.starts_with("@class") {
                line = line.replace("@class", "").trim().to_string();
                md_headings(line.clone(), &mut output, "# class");
                continue;
            }

            if line.starts_with("@method") {
                line = line.replace("@method", "").trim().to_string();
                md_headings(line.clone(), &mut output, "## method");
                continue;
            }

            if line.starts_with("@function") {
                line = line.replace("@function", "").trim().to_string();
                md_headings(line.clone(), &mut output, "## function");
                continue;
            }

            if line.starts_with("@param") {
                continue;
            }

            if line.starts_with("@example") {
                continue;
            }

            if line.starts_with("@returns") {
                continue;
            }

            output.push(line.clone());
        }

        return output.join("\r");
    }
}

fn md_headings(line: String, output: &mut Vec<String>, heading_text: &str) {
    let parts = line.split("-").collect::<Vec<&str>>();

    let mut heading: Vec<&str> = vec![heading_text, "-"];
    heading.push(parts[0].trim());

    output.push(heading.join(" "));
}
