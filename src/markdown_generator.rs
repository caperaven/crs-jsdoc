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

    // Stack maintaining states while processing comments and producing markdown
    stack: Vec<MarkdownGeneratorState>,

    // The lines of the markdown file
    lines: Vec<String>
}

impl MarkdownGenerator {
    pub fn new() -> MarkdownGenerator {
        MarkdownGenerator {
            current_state: MarkdownGeneratorState::None,
            stack: vec![],
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

    pub fn generate(self) {
        for (index, line) in self.lines.iter().enumerate() {
            println!("{}: {}", index, line);
        }
    }
}