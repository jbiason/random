import os, glob, telnetlib, time

CAMINHO = "/home/admin/deploy"
HOST = "127.0.0.1" # conecta no osgi do idempiere via localhost
PORT = 12612     # porta do osgi
TIMEOUT = 2


tn = telnetlib.Telnet(HOST, PORT, TIMEOUT)

# print("Open")
# tn.open(HOST, PORT)
print("ls")
tn.write("ls")
print("exit")
tn.write("exit")

tn.close()
print('Concluido')
