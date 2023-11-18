import socket
# Create a TCP socket
sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
# Connect to the server
server_address = ('127.0.0.1', 8080)
sock.connect(server_address)
# Send data
message = 'Hello, server!'
sock.sendall(message.encode())
# Receive response
response = sock.recv(1024)
print('Received:', response.decode())
# Close the socket
sock.close()
