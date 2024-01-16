import subprocess

def test():
    try:
        result = subprocess.run(["soroban", "config", "network", "ls"], check=True, capture_output = True)
        print(result.stdout.decode('ascii'))
    except:
        print("false")
        raise Exception("Error")

try:
    result = subprocess.run(["soroban", "config", "identity", "address", "--global", "telecom1"], check=True, capture_output = True)
except:
    raise Exception("Cannot find contract ID!")
contract = result.stdout.decode('ascii')[:-1]
print(contract)