import json
import socket

# https://stackoverflow.com/questions/48024720/python-how-to-check-if-socket-is-still-connected
def is_socket_closed(sock: socket.socket) -> bool:
    try:
        # this will try to read bytes without blocking and also without removing them from buffer (peek only)
        data = sock.recv(16, socket.MSG_DONTWAIT | socket.MSG_PEEK)
        if len(data) == 0:
            return True
    except BlockingIOError:
        return False  # socket is open and reading from it would block
    except ConnectionResetError:
        return True  # socket was closed for some other reason
    except Exception as e:
        return False
    return False


class JSONRPCException(Exception):
    def __init__(self, code, msg):
        self.code = code
        self.msg = msg
        super(JSONRPCException, self).__init__(msg)


class LagoonClient(object):

    MAX_ID = 2 << 16
    BUFF = 4096

    def __init__(self, host, port):
        self._host = host
        self._port = port
        self.sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.sock.connect((self._host, self._port))
        self._rpc_id = 1

    def reconnect(self):
        self.sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.sock.connect((self._host, self._port))

    def _invoke(self, id_value, method_name, *params):
        payload = {
            "jsonrpc": "2.0",
            "id": id_value % self.MAX_ID,
            "method": method_name,
            "params": list(params),
        }
        str_data = json.dumps(payload) + "\n"
        self.sock.send(str_data.encode("utf-8"))

    def _recieve(self):
        data = self.sock.recv(self.BUFF)
        str_data = data.decode("utf-8")
        result = json.loads(str_data[:-1])
        if "error" in result:
            raise JSONRPCException(
                result["error"]["code"], result["error"]["message"]
            )
        return result["result"]

    def invoke(self, id_value, method_name, *params):
        if is_socket_closed(self.sock):
            self.reconnect()
        self._invoke(id_value, method_name, *params)

    def recieve(self):
        if is_socket_closed(self.sock):
            self.reconnect()
        return self._recieve()

    def create_collection(self, collection_name):
        self.invoke(self._rpc_id, "createCollection", collection_name)
        data = self.recieve()
        self._rpc_id += 1
        return data

    def exist(self, collection, key):
        self.invoke(self._rpc_id, "hasKey", collection, key)
        data = self.recieve()
        self._rpc_id += 1
        return data

    def set_key(self, collection, key):
        self.invoke(self._rpc_id, "setKey", collection, key)
        data = self.recieve()
        self._rpc_id += 1
        return data


if __name__ == "__main__":
    lagoon = LagoonClient("localhost", 3030)
    print(lagoon.create_collection("hello"))
    print(lagoon.exist("hello", "key"))
    print(lagoon.set_key("hello", "key1"))
    print(lagoon.exist("hello", "key1"))
