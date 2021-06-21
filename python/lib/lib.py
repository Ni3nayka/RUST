from ctypes import cdll

def liba():
    return cdll.LoadLibrary("lib/target/release/lib.dll")
