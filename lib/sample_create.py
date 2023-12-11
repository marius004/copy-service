from config import PROJECT_DIRECTORY, DAEMON_HOST, DAEMON_PORT
from models.requests import *
from models.responses import *
from executor import Executor
import time

create_requests = [
    CreateOperation(
        source=f"{PROJECT_DIRECTORY}/lib/create.py",
        destination=f"{PROJECT_DIRECTORY}/daemon/tmp/create.py"),
    CreateOperation(
        source=f"{PROJECT_DIRECTORY}/src/client/client.rs",
        destination=f"{PROJECT_DIRECTORY}/daemon/tmp/client.rs"),
    CreateOperation(
        source=f"{PROJECT_DIRECTORY}/src/models/copy.rs",
        destination=f"{PROJECT_DIRECTORY}/daemon/tmp/copy-model.rs"),
    CreateOperation(
        source=f"{PROJECT_DIRECTORY}/src/models/job.rs",
        destination=f"{PROJECT_DIRECTORY}/daemon/tmp/job.rs"),
    CreateOperation(
        source=f"{PROJECT_DIRECTORY}/src/services/copy.rs",
        destination=f"{PROJECT_DIRECTORY}/daemon/tmp/copy.rs"),
    CreateOperation(
        source=f"{PROJECT_DIRECTORY}/src/services/storage.rs",
        destination=f"{PROJECT_DIRECTORY}/daemon/tmp/storage.rs"),
]

executor = Executor(DAEMON_HOST, DAEMON_PORT)

for request in create_requests:
    response = executor.exec(request)
    print(CreateResponse.from_json(response))

while True: 
    response = ListResponse.from_json(executor.exec(ListOperation()))
    
    if len(response) == 0: 
        break
    
    for job in response: 
        print(f"{job.id} -> {job.writes} writes", end="; ")
    
    print()
    time.sleep(1)