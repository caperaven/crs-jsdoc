pub struct Args {
    pub file: Option<String>,
    pub folder: Option<String>,
    pub destination: Option<String>,
}

/**
 * Get the file from the commandline args
 */
pub(crate) fn get_args() -> Args {
    let args: Vec<String> = std::env::args().collect();

    Args {
        file: get_values_from_args(&args, "file"),
        folder: get_values_from_args(&args, "folder"),
        destination: get_values_from_args(&args, "dest"),
    }
}

/**
 * Get the arg details from the commandline args
 */
pub(crate) fn get_values_from_args(args: &Vec<String>, search_arg: &str) -> Option<String> {
    args.iter().position(|arg| arg == search_arg)
        .and_then(|index| args.get(index + 1))
        .map(|result| result.to_string())
}