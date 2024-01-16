from flask import Flask, request
from transaction import TransactionClass
from user import UserClass
from clear import ClearClass

app = Flask(__name__)

@app.route('/')
def hello():
    return 'Hello, World!'

@app.route('/test')
def test():
    transaction = TransactionClass("telecom1")
    return transaction.test()

#################### Transaction ####################
# data: {user, telecom_pay, usage}
@app.route('/<telecom>/transaction/add', methods=['POST'])
def add_transaction(telecom):
    if request.method == 'POST':
        transaction = TransactionClass(telecom)
        data = request.get_json()
        try:
            transaction.add(data["user"], data["telecom_pay"], telecom, data["usage"])
            return 'Success', 200
        except:
            return 'Fail', 403

@app.route('/<telecom>/transaction/get', methods=['GET'])
def get_transactions(telecom):
    if request.method == 'GET':
        transaction = TransactionClass(telecom)
        try:
            result = transaction.get_all(telecom)
            return result, 200
        except:
            return 'Fail', 403

@app.route('/<telecom>/transaction/remove', methods=['POST'])
def remove_transactions(telecom):
    if request.method == 'POST':
        transaction = TransactionClass(telecom)
        try:
            transaction.remove_all(telecom)
            return 'Success', 200
        except:
            return 'Fail', 403

####################### User ########################
@app.route('/<telecom>/user_usage/get', methods=['GET'])
def get_user_usage(telecom):
    if request.method == 'GET':
        user = UserClass(telecom)
        try:
            result = user.get(telecom)
            return result, 200
        except:
            return 'Fail', 403
        
@app.route('/<telecom>/user/remove', methods=['POST'])
def remove_user_usage(telecom):
    if request.method == 'POST':
        user = UserClass(telecom)
        try:
            user.remove(telecom)
            return 'Success', 200
        except:
            return 'Fail', 403
        
####################### Clear #######################
@app.route('/<telecom>/clear/get', methods=['GET'])
def get_clear(telecom):
    if request.method == 'GET':
        clear = ClearClass(telecom)
        try:
            result = clear.get(telecom)
            return result, 200
        except:
            return 'Fail', 403
        
@app.route('/<telecom>/clear/remove', methods=['POST'])
def remove_clear(telecom):
    if request.method == 'POST':
        clear = ClearClass(telecom)
        try:
            clear.remove(telecom)
            return 'Success', 200
        except:
            return 'Fail', 403

# run "python3 server.py" to start development server
# run "gunicorn server:app" to start production depployment server
if __name__ == '__main__':
    app.run(host='0.0.0.0', port=8000, debug=True, use_reloader=False)