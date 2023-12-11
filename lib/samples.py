from config import PROJECT_DIRECTORY, DAEMON_HOST, DAEMON_PORT
from models.requests import *
from models.responses import *
from executor import Executor
import time
import os

class Test():
    def __init__(self, executor: Executor): 
        self.executor = executor
    
    def create_multiple_files(self):
        requests = [
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

        for request in requests:
            self.executor.exec(request)
        
        for _ in range(120): 
            for job in ListResponse.from_json(self.executor.exec(ListOperation())):
                print(f"{job.id} -> {job.writes}")
            time.sleep(0.5)
            os.system("clear")
    
    def create_source_path_does_not_exist(self):        
        requests = [
            CreateOperation(
                source=f"{PROJECT_DIRECTORY}/does-not-exist.txt",
                destination=f"{PROJECT_DIRECTORY}/daemon/tmp/copy.txt"),
        ]

        for request in requests:
            create_response = CreateResponse.from_json(self.executor.exec(request))
            print(create_response)

            response = ProgressResponse.from_json(self.executor.exec(ProgressOperation(create_response.job_id)))
            print(response)

test = Test(Executor(DAEMON_HOST, DAEMON_PORT))

test.create_multiple_files()
# test.create_source_path_does_not_exist()