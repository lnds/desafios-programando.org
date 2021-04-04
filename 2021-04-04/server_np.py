import os
import os.path
import time 

def server():
    fifo = "/tmp/fifo_twoway"

    if os.path.exists(fifo):
        os.remove(fifo)

    max_line = 80
    mode = 0o640

    os.mkfifo(fifo, mode)

    print('server')
    pipe = os.open(fifo, os.O_RDWR)
    while True:
        s = os.read(pipe, max_line)
        print('recibido:', s)
        if s == b'end':
            os.close(pipe)
            return
        else:
            rev = s[::-1]
            os.write(pipe, rev)
            time.sleep(2)

if __name__ == "__main__":
    server()
