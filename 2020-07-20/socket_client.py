# socket_client.py
import socket
import sys

sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

port = 12345
server_address = ('localhost', port)
sock.connect(server_address)

try:
    msg = sys.argv[1]
    sock.sendall(msg)

    received = 0
    msg_len = len(msg)
    
    while received < msg_len:
        data = sock.recv(16)
        print data
        received += len(data)
finally:
    sock.close()
