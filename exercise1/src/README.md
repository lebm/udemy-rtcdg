https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=465b0583405796aebccd3e07e97784d6

As expected, on the first call of take_ownership the value of account is moved to the function, so it became unitialized on the main function and the code does not compile.
One solution is to pass a "borrowed" value to the function.
