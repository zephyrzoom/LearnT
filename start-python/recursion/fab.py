result = []
def fab(n):
    if n == 2:
        result.append(1)
        result.append(1)
    else:
        fab(n-1)
        result.append(result[n-2]+result[n-3])

if __name__ == "__main__":
    fab(50)
    print(result)