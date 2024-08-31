# read from touch sensor

import serial
from timeit import default_timer as timer

ser = serial.Serial('COM3', 115200)
ser.write("{STAT}".encode())

while True:
   start = Timer()
   x = ser.read(9)
   end = Timer()
   print(x)
   #print(end-start)
