import subprocess
import random

network_id = "CAAGS6S6NCH3SY2NUJJKVGMA5DFMLFKYSIPE6GAYOZ42XGBOGLRPVHIP"

User = {}
Clear = {}
for i in range(101, 201):
    user = "User" + str(random.randrange(1, 11))
    usage = random.randrange(1, 1001)
    telecom_num = random.sample(range(1, 4), 2)
    telecom = ["Telecom" + str(t) for t in telecom_num]

    if user not in User:
        User[user] = usage
    else:
        User[user] += usage

    if telecom[0] not in Clear:
        Clear[telecom[0]] = -usage * 100
    else:
        Clear[telecom[0]] -= usage * 100
    
    if telecom[1] not in Clear:
        Clear[telecom[1]] = usage * 100
    else:
        Clear[telecom[1]] += usage * 100

    subprocess.run(["soroban", "contract", "invoke", 
                    "--id", network_id, 
                    "--source", "telecom0",
                    "--network", "localhost_testnet",
                    "--", 
                    "add_transaction", 
                    "--id", "test_" + str(i),
                    "--user", user,
                    "--telecom_pay", telecom[0],
                    "--telecom_receive", telecom[1],
                    "--usage", str(usage)])
    
f = open("./test_result.txt", "w")
f.write(str(dict(sorted(User.items()))) + "\n" + str(dict(sorted(Clear.items()))))
f.close()