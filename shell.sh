#!/bin/bash

# Ensure that out.ll exists
if [ ! -f out.ll ]; then
    echo "LLVM IR file 'out.ll' does not exist!"
    exit 1
fi



# Option 1: Run the LLVM IR directly using lli (LLVM Interpreter)
cargo run $1 $2  --quiet> out.ll
echo "Compiling out.ll to machine code..."
clang out.ll -w -o out  # Running directly from .ll file
./out
# Option 2: Alternatively, you can compile the bitcode to a native executable using clang
# Uncomment the next two lines if you want to use clang to generate an executable:
# echo "Compiling bitcode to executable using clang..."
# clang out.bc -o out_executable

# Check if clang compilation was successful
# if [ $? -eq 0 ]; then
#     echo "Running the compiled executable..."
#     ./out_executable  # Run the native executable
# else
#     echo "clang failed to 
