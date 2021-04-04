import os
import time

def client():
    fifo = "/tmp/fifo_twoway"

    print('cliente')
    pipe = os.open(fifo, os.O_RDWR)
    while True:
        s = input('Ingrese un string: ')
        if s == "end":
            os.write(pipe, b'end')
            os.close(pipe)
            return
        else:
            os.write(pipe, bytearray(s, 'utf-8'))
            rev = os.read(pipe, len(s))
            print('Reverse:', rev.decode('utf-8'))


if __name__ == "__main__":
    client()
