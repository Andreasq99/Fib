import time

def fib1(n):
    if(n<=1):
        return n
    else:
        return fib1(n-1) + fib1(n-2)

def fib2(n):
    a = 0
    b = 1
    next = 0

    while(n>0):
        next = a + b
        a = b
        b = next
        n = n - 1
    return a


def main():
    start = time.time()
    for x in range(31):
        print(fib2(x))
    print(time.time() - start)

main()