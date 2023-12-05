import socket

def send_message(message):
    host = "127.0.0.1"  
    port = 8080 

    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect((host, port))
        s.sendall(message.encode())

operations = {
    "copy": '''
    {
        "request_type": "copy", 
        "source_path": "", 
        "destination_path": ""
    }
    '''
}

send_message(operations["copy"])