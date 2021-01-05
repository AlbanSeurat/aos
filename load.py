import sys
import serial
import os
import time

kernel_img = "kernel-high.img"

def ack(ser):
   i = 0
   while i < 3:
      c = ser.read(1)
      print(c)
      if c == b'\x03':
         i += 1

def send_kernel(ser, data):
   while data:
      sent = ser.write(data)
      data = data[sent:]


def create_serial() :
   ser = serial.Serial()
   ser.port=sys.argv[1]
   ser.baudrate=115200
   ser.open()
   return ser

def read_short(ser):
   return int.from_bytes(ser.read(2), byteorder='big', signed=False)

def read_kernel():
   f = open(kernel_img, "rb")
   data = f.read()
   f.close()
   return data

s = create_serial()
ack(s)

kernel_size = os.path.getsize(kernel_img)
print(f"write {kernel_size:d}")
s.write(kernel_size.to_bytes(2, byteorder='big'))
ack(s)

print(f"kernel size received {read_short(s):d}")
ack(s)

print("send kernel")
send_kernel(s, read_kernel())
ack(s)

print("kernel sent")
while 1:
   print(s.readline())

