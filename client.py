import socket

# Socket path
socket_path = '/tmp/nexsock.sock'

# Create client socket
client = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
client.connect(socket_path)

# Send message
message = "Hello, Unix socket!"
client.send(message.encode())

# Receive response
response = client.recv(1024).decode()
print(f"Server response: {response}")

client.close()