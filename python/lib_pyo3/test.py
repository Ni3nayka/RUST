'''
from ctypes import cdll

#lib = cdll.LoadLibrary("lib_pyo3/target/release/lib_pyo3.dll")
lib = cdll.LoadLibrary("lib_pyo3/target/release/string_sum.dll")

a = "qwerty"
b = "asdfgh"

answer = lib.string_sum(a,b)

print(answer)
'''

import string_sum

answer = string_sum.sum_as_string(1,2)

print(answer)
