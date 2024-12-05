def fibonacci(n):
    fib_sequence = [0, 1]  # Starting with the first two Fibonacci numbers
    for i in range(2, n):
        fib_sequence.append(fib_sequence[i - 1] + fib_sequence[i - 2])  # Add the next number in the sequence
    return fib_sequence

# Get the first 40 Fibonacci numbers
n = 1000
fib_numbers = fibonacci(n)

# Print the Fibonacci numbers
print(f"The first {n} Fibonacci numbers are:")
for num in fib_numbers:
    print(num)
