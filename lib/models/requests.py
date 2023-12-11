from abc import ABC, abstractmethod
from dataclasses import dataclass
from enum import Enum

class OperationType(Enum): 
    CREATE = 0, 
    SUSPEND = 1,
    CANCEL = 2,
    PROGRESS = 3, 
    LIST = 4

class BaseOperation(ABC): 
    @abstractmethod
    def type(self) -> OperationType:
        pass

    @abstractmethod
    def repr(self) -> str:
        pass

@dataclass
class CreateOperation(BaseOperation):
    def __init__(self, source: str, destination: str): 
        self.source = source
        self.destination = destination
    
    def type(self) -> OperationType:
        return OperationType.CREATE

    def repr(self) -> str:
        return f'{{"request_type": "copy", "source_path": "{self.source}", "destination_path": "{self.destination}"}}'


@dataclass
class SuspendOperation(BaseOperation):
    def __init__(self, job_id: str): 
        self.job_id = job_id
    
    def type(self) -> OperationType:
        return OperationType.SUSPEND

    def repr(self) -> str:
        return f'{{"request_type": "suspend", "job_id": "{self.job_id}"}}'

@dataclass
class CancelOperation(BaseOperation):
    def __init__(self, job_id: str): 
        self.job_id = job_id
    
    def type(self) -> OperationType:
        return OperationType.CANCEL

    def repr(self) -> str:
        return f'{{"request_type": "cancel", "job_id": "{self.job_id}"}}'
    
@dataclass
class ProgressOperation(BaseOperation):
    def __init__(self, job_id: str): 
        self.job_id = job_id
    
    def type(self) -> OperationType:
        return OperationType.PROGRESS

    def repr(self) -> str:
        return f'{{"request_type": "progress", "job_id": "{self.job_id}"}}'

@dataclass
class ListOperation(BaseOperation):
    def type(self) -> OperationType:
        return OperationType.LIST

    def repr(self) -> str:
        return f'{{"request_type": "list"}}'