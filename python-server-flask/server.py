from flask import Flask
import time;

app = Flask(__name__)

@app.route("/sleep/<n>")
def sleep(n):
    n = int(n)
    x = n * 2
    # time.sleep(n)
    time.sleep(1)
    return f"<p>{n} -> {x}</p>"
