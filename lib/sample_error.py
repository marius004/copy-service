from config import PROJECT_DIRECTORY, DAEMON_HOST, DAEMON_PORT
from models.requests import *
from models.responses import *
from executor import Executor

create_requests = [
    CreateOperation(
        source=f"{PROJECT_DIRECTORY}/does-not-exist.txt",
        destination=f"{PROJECT_DIRECTORY}/daemon/tmp/copy.txt"),
]

executor = Executor(DAEMON_HOST, DAEMON_PORT)

for request in create_requests:
    create_response = CreateResponse.from_json(executor.exec(request))
    print(create_response)

    response = ProgressResponse.from_json(executor.exec(ProgressOperation(create_response.job_id)))
    print(response)