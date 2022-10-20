import socket
from enum import Enum

class Value(Enum):
    String = (1).to_bytes(4, byteorder='big')
    Int = (2).to_bytes(4, byteorder='big')

class OperationType(Enum):
    SET = (1).to_bytes(4, byteorder='big')
    GET = (2).to_bytes(4, byteorder='big')
    DELETE = (3).to_bytes(4, byteorder='big')

TPC_IP = "127.0.0.1"
TPC_PORT = 9990
BUFFER_SIZE = 1024

KEY = "test".encode()
KEY_DEAD_SPACE = bytearray(20 - len(KEY))

VALUE = "test".encode()
VALUE_DEAD_SPACE = bytearray(20 - len(VALUE))

MESSAGE = OperationType.GET.value + Value.String.value + KEY + KEY_DEAD_SPACE + VALUE + VALUE_DEAD_SPACE
s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s.connect((TPC_IP, TPC_PORT))
s.send(MESSAGE)
data = s.recv(BUFFER_SIZE)
print(data.decode())
s.close()