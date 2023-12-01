#[derive(Debug)]
pub struct Job {
    source: &'static str,
    destination: &'static str,
    status: JobStatus,
}

#[derive(Debug)]
pub enum JobStatus {
    Created,
    Running,
    Suspended,
    Completed,
    Canceled,
}

impl Job {
    fn new(source: &'static str, destination: &'static str) -> Self {
        Job {
            source: source, 
            destination: destination,
            status: JobStatus::Created,
        }
    }
}
