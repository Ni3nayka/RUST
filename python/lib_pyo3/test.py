from ctypes import cdll

lib = cdll.LoadLibrary("lib_pyo3/target/release/lib_pyo3.dll")

a = "qwerty"
b = "asdfgh"

answer = lib.sum_as_string(a,b)

print(answer)
