from Crypto.Cipher import AES
import codecs

def decryp(pwd):
    print("#######################")
    print(pwd)
    unpad = lambda s : s[:-(s[-1] if type(s[-1]) == int else ord(s[-1]))]
    mode = AES.MODE_CBC  
    cipher = AES.new(b'1111111111111111',mode,b'1111111111111111')  
    decrypted = unpad(cipher.decrypt(bytes(bytearray.fromhex(pwd))))
    print("#######################")
    return(decrypted)

result = decryp('c9115981d982b8ed8ec80b0ceb42644a')
print(result)