def fib(n):
    a = 0
    b = 1
    next = 0

    while(n>0):
        next = a + b
        a = b
        b = next
        n = n - 1
    return a