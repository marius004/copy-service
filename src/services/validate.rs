use std::{fs, sync::Arc, path::Path};

use crate::models::job::Job;

type ValidationResult = (bool, String);

fn are_paths_pointing_to_same_entity(source_path: &str, destination_path: &str) -> bool {
    // same as source_meta.ino() == destination_meta.ino() && source_meta.dev() == destination_meta.dev()
    // but this approach also works for directories

    match (fs::canonicalize(source_path), fs::canonicalize(destination_path)) {
        (Ok(source_canonical), Ok(destination_canonical)) => {
            source_canonical == destination_canonical
        }
        _ => false,
    }
}


pub fn validate(job: Arc<Job>) -> ValidationResult {
    if !Path::new(&job.source).exists() {
        return (false, String::from("Source path does not exist"));
    }
    if are_paths_pointing_to_same_entity(&job.source, &job.destination) {
        return (false, String::from("Source and destination point to the same entity"))
    } 

    (true, String::from(""))
}
