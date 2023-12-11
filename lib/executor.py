from models.requests import BaseOperation 
from typing import List
import socket
import json

class Executor: 
    def __init__(self, host: str, port: int): 
        self.host = host 
        self.port = port 
    
    def exec(self, operation: BaseOperation) -> str: 
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            s.connect((self.host, self.port))
            s.sendall(operation.repr().encode())
            return json.loads(s.recv(4096).decode()) 
    
    def exec_bulk(self, operations: List[BaseOperation]) -> List[str]: 
        return [self.exec(operation) for operation in operations]