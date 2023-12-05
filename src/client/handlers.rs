use std::collections::HashSet;

use crate::client::requests::*;
use crate::services::copy::CopyService;
use crate::models::job::Job;

pub fn handle_copy(request: CopyJobRequest, copy_service: &mut CopyService) {
    let job = Job::new(
        request.source_path,
        request.destination_path,
        None, 
        HashSet::new());

    copy_service.add_job(job);
    
    // todo: execute should not be here!
    // todo: copy_service.execute should be on its own thread!
    copy_service.execute();
}