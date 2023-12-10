import socket
import json 

def receive_message(socket):
    return json.loads(socket.recv(4096)) 

def send_message(message):
    host = "127.0.0.1"  
    port = 8080 

    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect((host, port))
        s.sendall(message.encode())
        print("Received response: ", receive_message(s))
        
operations = {
    "copy": '''
    {
        "request_type": "copy", 
        "source_path": "/home/smarius/Documents/copy-service/src/services/copy.rs", 
        "destination_path": "/home/smarius/Documents/copy-service/copy.rs"
    }
    ''',
    "list": '''
    {
        "request_type": "list"
    }
    '''
}

send_message(operations["copy"])
for i in range(60):
    send_message(operations["list"])