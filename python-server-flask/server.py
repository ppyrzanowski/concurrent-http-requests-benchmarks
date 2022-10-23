from flask import Flask
import time;

app = Flask(__name__)

@app.route("/sleep/<n>")
def sleep(n):
    x = int(n) * 2
    time.sleep(1)
    return f"<p>{n} -> {x}</p>"
