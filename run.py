import time
import sys

import programs.fib0 as fib0
import programs.fib1 as fib1

options = [fib0.fib, fib1.fib]
option = int(sys.argv[1])
fib = options[option]
limit = int(sys.argv[2])

def main():
    start = time.time()
    for x in range(limit):
        print(fib(x))
    print(time.time() - start)

main()