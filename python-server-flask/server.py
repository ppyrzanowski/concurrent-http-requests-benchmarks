from flask import Flask
import time;

app = Flask(__name__)

@app.route("/sleep/<n>")
def sleep(n):
    time.sleep(int(n))
    return '<p styles="color=red">Hello world!</p>'
