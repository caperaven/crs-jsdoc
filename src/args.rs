/**
 * The structure that holds the commandline arguments
 */
pub struct Args {
    pub file: Option<String>,
    pub folder: Option<String>,
    pub destination: Option<String>,
}

/**
 * Get the commandline args and return them as a struct
 */
pub(crate) fn get_args() -> Args {
    let args: Vec<String> = std::env::args().collect();

    let file = get_values_from_args(&args, "file");
    let folder = get_values_from_args(&args, "folder");
    let mut destination = get_values_from_args(&args, "dest");

    if destination.is_none() {
        destination = Some("documents".to_string());
    }

    Args {
        file,
        folder,
        destination
    }
}

/**
 * Get the arg details from the commandline args
 * @param args The commandline args as a vector
 * @param search_arg The argument to search for in the vector
 */
pub(crate) fn get_values_from_args(args: &Vec<String>, search_arg: &str) -> Option<String> {
    args.iter().position(|arg| arg == search_arg)
        .and_then(|index| args.get(index + 1))
        .map(|result| result.to_string())
}