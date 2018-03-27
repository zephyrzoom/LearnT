from Crypto.Cipher import AES
import codecs

def decryp(pwd):
    print("#######################")
    print(pwd)
    unpad = lambda s : s[:-(s[-1] if type(s[-1]) == int else ord(s[-1]))]
    mode = AES.MODE_CBC  
    cipher = AES.new('1111111111111111',mode,'1111111111111111')  
    decrypted = unpad(cipher.decrypt(bytes(bytearray.fromhex(pwd))))
    print("#######################")
    return(decrypted)

print(decryp('c9115981d982b8ed8ec80b0ceb42644a'))