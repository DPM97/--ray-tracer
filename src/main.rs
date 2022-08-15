use clap::{App, Arg};
use rust_ray_tracer::img::create_png;
use std::path::PathBuf;

fn main() {
    let res = App::new("rust_ray_tracer")
        .arg(
            Arg::with_name("out_path")
                .index(1)
                .validator(validate_out_file_path),
        )
        .try_get_matches();

    // throw err if parsing or validation failed
    if res.is_err() {
        res.unwrap_err().exit();
    }

    let res = res.unwrap();
    let path_as_str = res.value_of("out_path");
    match path_as_str {
        Some(x) => create_png(PathBuf::from(x)),
        None => panic!("failed to cast path str to PathBuf"),
    }
}

fn validate_out_file_path(v: &str) -> Result<(), String> {
    if v.ends_with(".png") {
        return Ok(());
    }
    Err(String::from(
        "the out file must use the png format (e.g., ./foo/bar.png)",
    ))
}

#[cfg(test)]
mod validation_tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_no_png() {
        assert!(validate_out_file_path("./test").is_err());
    }

    #[test]
    fn test_png() {
        assert!(validate_out_file_path("./test.png").is_ok());
    }
}