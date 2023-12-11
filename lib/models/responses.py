from dataclasses import dataclass
from typing import List
from enum import Enum

class ResponseType(Enum):
    CREATE = 0
    SUSPEND = 1
    CANCEL = 2
    PROGRESS = 3
    LIST = 4

@dataclass
class BaseResponse:
    type: ResponseType
    error: str

@dataclass
class CreateResponse(BaseResponse):
    job_id: str

    @staticmethod
    def from_json(obj: dict) -> 'CreateResponse':
        if obj.get("message", None) is not None: 
            return CreateResponse(type=ResponseType.CREATE, error=obj["message"], job_id="")
        return CreateResponse(type=ResponseType.CREATE, error=None, job_id=obj.get("job_id", None))

@dataclass
class SuspendResponse(BaseResponse):
    message: str

    @staticmethod
    def from_json(obj: dict) -> 'SuspendResponse':
        if "error" in obj.get("message", "").lower(): 
            return SuspendResponse(type=ResponseType.SUSPEND, error=obj["message"])
        return SuspendResponse(type=ResponseType.SUSPEND, message=obj["message"])

@dataclass
class CancelResponse(BaseResponse):
    message: str

    @staticmethod
    def from_json(obj: dict) -> 'CancelResponse':
        if "error" in obj.get("message", "").lower(): 
            return CancelResponse(type=ResponseType.CANCEL, error=obj["message"])
        return CancelResponse(type=ResponseType.CANCEL, message=obj["message"])

@dataclass
class JobResponse(BaseResponse):
    id: str
    source: str
    destination: str 
    status: str
    writes: str
    
    @staticmethod
    def from_json(obj: dict, type: ResponseType) -> 'JobResponse':
        return JobResponse(
            type=type,
            error=obj.get('message', None),
            id=obj.get('id', None),
            source=obj.get('source', None),
            destination=obj.get('destination', None),
            status=obj.get('status', None),
            writes=obj.get('writes', None)
        )
    
@dataclass
class ProgressResponse(JobResponse):
    @staticmethod
    def from_json(obj: dict) -> 'ProgressResponse':
        return JobResponse.from_json(obj, ResponseType.PROGRESS)

@dataclass
class ListResponse(BaseResponse):
    jobs: List[JobResponse]

    @staticmethod
    def from_json(obj_list: List) -> 'ListResponse':
        return [JobResponse.from_json(obj, ResponseType.LIST) for obj in obj_list]