As expected, on the first call of take_ownership the value of account is moved to the function, so it became unitialized on the main function and the code does not compile.
One solution is to pass a "borrowed" value to the function.
