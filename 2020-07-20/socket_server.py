# socket_server.py

import socket
import sys

def process_connection(connection, client_address):
    try:
        print 'conexion desde la direccion:', client_address
	print "data:\n"        
        while True:
            data = connection.recv(16)
            if data:
		print data
                connection.sendall(data)
            else:
                break
    finally:
        connection.close()

sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

port = 12345

server_address = ('localhost', port)
print 'iniciando servidor en puerto', port

sock.bind(server_address)


sock.listen(1)

while True:
    connection, client_address = sock.accept()
    process_connection(connection, client_address)
