from Crypto.Cipher import AES
from Crypto.Util.Padding import pad, unpad
import codecs

def decryp(pwd):
    print("#######################")
    print(pwd)
    #unpad = lambda s : s[:-(s[-1] if type(s[-1]) == int else ord(s[-1]))]

    mode = AES.MODE_CBC  
    cipher = AES.new(b'1111111111111111',mode,b'1111111111111111')  
    decrypted = unpad(cipher.decrypt(bytes(bytearray.fromhex(pwd))), 16)
    print("#######################")
    return(decrypted)


def crypt():
    cipher = AES.new(b'1111111111111111', AES.MODE_CBC, b'1111111111111111')
    msg = pad(b'my message', 16)
    ciphertext = cipher.encrypt(msg)
    print(ciphertext.hex())

result = decryp('c9115981d982b8ed8ec80b0ceb42644a')
print(result)

crypt()