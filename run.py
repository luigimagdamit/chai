import subprocess

# Prompt the user for a command to run

while True:
    command = input(">")

    compiler = "./shell.sh cmd " + "\"" + command + "\""
    print(compiler)
    # Run the command and wait for it to complete
    print(compiler)
    exit_code = subprocess.call(compiler, shell=True)
