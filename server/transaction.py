import subprocess

class TransactionClass:
    def __init__(self, telecom):
        try:
            contract_id = subprocess.run(["cat", "../.soroban/arch-id"], check=True, capture_output = True)
        except:
            raise Exception("Cannot find contract ID!")
        try:
            auth_id = subprocess.run(["soroban", "config", "identity", "address", "--global", telecom], check=True, capture_output = True)
        except:
            raise Exception("Cannot find auth ID!")
        
        self.contract = contract_id.stdout.decode('ascii')[:-1]
        self.auth = auth_id.stdout.decode('ascii')[:-1]

    def add(self, user, telecom_pay, telecom_receive, usage):
        try:
            subprocess.run(["soroban", "contract", "invoke", 
                        "--id", self.contract, 
                        "--source", telecom_receive,
                        "--network", "testnet",
                        "--", 
                        "add_transaction", 
                        "--auth", self.auth,
                        "--user", user,
                        "--telecom_pay", telecom_pay,
                        "--telecom_receive", telecom_receive,
                        "--usage", str(usage)], check=True)
        except:
            raise Exception("Error")
        
    def get_all(self, telecom):
        try:
            result = subprocess.run(["soroban", "contract", "invoke", 
                        "--id", self.contract, 
                        "--source", telecom,
                        "--network", "testnet",
                        "--", 
                        "get_all_transactions", 
                        "--auth", self.auth], check=True, capture_output = True)
            return result.stdout.decode('ascii')
        except:
            raise Exception("Error")
        
        
    def remove_all(self, telecom):
        try:
            subprocess.run(["soroban", "contract", "invoke", 
                            "--id", self.contract, 
                            "--source", telecom,
                            "--network", "testnet",
                            "--", 
                            "remove_transactions", 
                            "--auth", self.auth], check=True)
        except:
            raise Exception("Error")