# cppcheckr
A small cpp file tester for competitive programing, running tests from a directory in .in and .out format :)
## usage

cppcheckr 
  -a *(path to algorithm)* without the .cpp
  -i testy/input_folder/test_prefix 
  -o /testy/output_folder/test_prefix 
  -b *(number of first test)* 
  -e *(number of last test )*

will:
- compile file *(path to algorithm)*.cpp
- run the program with input from folder *testy/input_folder* and compare it to outputs from folder *testy/output_folder*
  - Where the name of the tests look like: 
    - testy/input_folder/test{i}.in
    - testy/outpu_folder/test{i}.out 
  - i being a number between *(number of first test)* and *(number of last test)*
- test the speed of those programs *( so you can see how well your optimizations work :) )*
- You can pass the argument -p 1 to make sure all tests pass :) **(but quality will not be guarantied)**

**more info in cppchecker -h**
