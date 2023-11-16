from pysyr import collatz, collatz_inc, collatz_pow, find_next
import sys
from multiprocessing.pool import Pool
import random
import locale
import time
import progressbar
locale.setlocale(locale.LC_ALL, '')

import pandas as pd
min_iter = 301
max_iter = 325
res = [int] * ((max_iter-min_iter)+1)
res_iter = [int] * ((max_iter-min_iter)+1)
exponents = [int] * ((max_iter-min_iter)+1)
idx = 0
for k in progressbar.progressbar(range(min_iter,max_iter+1)):

    #p=pow(2,k)
    i = 0
    p=k
        #print(f"Begining for 2^{p} + "+str(2*i+1))
    num_iter, div, mul = collatz_pow(2,p,2*i+1,False)
    basic_num_iter = num_iter
    num_iter_str= "{0:n}".format(num_iter)
    #print(f"Collatz is true for 2^{p} + {2*i+1} with {num_iter_str} iterations")
    num_iter = find_next(2,p)
    exponents[idx] = p
    res[idx] = 2*i+1
    res_iter[idx] = num_iter
    num_iter= f"{num_iter:n}"
    div = "{0:n}".format(div)
    mul = "{0:n}".format(mul)
    #print(f"Collatz is true for 2^{p} + {2*i+1} with {num_iter} iterations: {div} divisions and {mul} multiplications")
    idx +=1
df = pd.DataFrame(list(zip(exponents,res, res_iter)),columns = ['pow_of_2','decay','new_num_iter'])
df.to_csv("./results_"+str(min_iter)+"_"+str(max_iter)+".csv")

"""
power = 1000000
my_number = pow(2,power)
out = "2^"+"{0:n}".format(power)
i = random.randint(1,200000000)
i = 2 * i + 1
out += " + "+"{0:n}".format(i)

print("n = "+ out)
my_number +=i
num_iter, div, mul = collatz(str(my_number))
num_iter= "{0:n}".format(num_iter)
div = "{0:n}".format(div)
mul = "{0:n}".format(mul)

print(f"Collatz is true for n with {num_iter} iterations:\n{div} divisions and {mul} multiplications")
"""
powers = []
"""
for i in range(23,40):
    fr = str(pow(2,i))
    to = str(pow(2,i+1))
    powers.append((fr,to))
    res = collatz_inc(fr,to)
    print(f"Collatz for n < 2^{i+1} is {res}")
    """
