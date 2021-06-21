'''
from ctypes import cdll

lib = cdll.LoadLibrary("lib/target/release/lib.dll")
lib.process()

print("сделано!")
'''

from lib import *

lib = liba()
lib.process()

print("сделано!")
