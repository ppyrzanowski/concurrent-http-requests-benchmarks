import requests;
import time;
import threading

url = "http://127.0.0.1:5000/sleep/"
# url = "http://httpbin.org/delay/5"
n = 20

session = requests.session()
def thread_function(i):
    response = session.get(url + "10")
    print(response.headers)
    time.sleep(1)
    # session.close()


time.sleep(2)
threads = list()
for i in range(n):
    t = threading.Thread(target=thread_function, args=(i,))
    threads.append(t)
    t.start()

for index, thread in enumerate(threads):
    thread.join()

print("Done")
