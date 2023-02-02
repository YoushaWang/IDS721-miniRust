from flask import Flask, request, jsonify

app = Flask(__name__)
tasks = ["task","Finish writing code"]

@app.route("/")
def index():
    return "Welcome to the To-Do List Microservice"

@app.route("/tasks", methods=["GET"])
def get_tasks():
    return jsonify(tasks)

@app.route("/tasks", methods=["POST"])
def add_task():
    task = request.get_json()
    tasks.append(task)
    return "Task added", 201

# from flask import Flask, request, jsonify
# app = Flask(__name__)

# tasks = ["1","2"]

# @app.route("/")
# def hello():
#     return "Welcome to the To-Do List Microservice"

# @app.route("/tasks", methods=["GET", "POST"])
# def task_list():
#     if request.method == "GET":
#         return jsonify(tasks)
#     elif request.method == "POST":
#         task = request.get_json()
#         tasks.append(task)
#         return jsonify(task), 201

# @app.route("/tasks/<int:task_id>", methods=["GET", "PUT", "DELETE"])
# def task_detail(task_id):
#     task = next((t for t in tasks if t["id"] == task_id), None)
#     if not task:
#         return "Task not found", 404
#     if request.method == "GET":
#         return jsonify(task)
#     elif request.method == "PUT":
#         task.update(request.get_json())
#         return jsonify(task)
#     elif request.method == "DELETE":
#         tasks.remove(task)
#         return "", 204
# testing
# def change(amount):
#     # Compute the resultant change and then store it to the result (res)
#     res = []
#     coins = [1,5,10,25] # It represents value of pennies, nickels, dimes, and quarters
#     coin_lookup = {25: "Quarters", 10: "Dimes", 5: "Nickels", 1: "Pennies"}

#     # Divide the amount (in cents) *100 by a coin value
#     # Record the quantity of evenly divided coins and remainder
#     coin = coins.pop()
#     num, rem  = divmod(int(amount*100), coin)
#     # Append the coin kinds and the total quantity of coins with no remaining value
#     res.append({num:coin_lookup[coin]})

#     # If there is still some remainder, keep adding coins to the result
#     while rem > 0:
#         coin = coins.pop()
#         num, rem = divmod(rem, coin)
#         if num:
#             if coin in coin_lookup:
#                 res.append({num:coin_lookup[coin]})
#     return res

# @app.route('/change/<dollar>/<cents>')
# def changeroute(dollar, cents):
#     print(f"Make Change for {dollar}.{cents}")
#     amount = f"{dollar}.{cents}"
#     result = change(float(amount))
#     return jsonify(result)

if __name__ == "__main__":
    app.run(host='0.0.0.0', port=8080,debug=True)
