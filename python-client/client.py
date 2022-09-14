import requests
import threading
import logging
import time
from timeit import default_timer as timer
from datetime import timedelta

def thread_function(name):
    # logging.info("Thread %s: starting", name)

    # url = "http://167.235.133.26:80/delay/2"
    url = f"http://127.0.0.1:5000/sleep/{name}"
    ses = requests.session()

    res = ses.get(url)
    # logging.info(f"Thread %s: response {res.text}", name)


def main():
    t = 500
    format = "%(asctime)s.%(msecs)03d: %(message)s"
    logging.basicConfig(format=format, level=logging.INFO, datefmt="%H:%M:%S")

    logging.info(f"Main    : creating {t} threads")

    threads = list()

    start = timer()
    for index in range(t):
        # logging.info("Main    : create and start thread %d.", index)
        x = threading.Thread(target=thread_function, args=(index,))
        threads.append(x)
        x.start()
    logging.info("Main    : done creating threads")

    for index, thread in enumerate(threads):
        # logging.info("Main    : before joining thread %d.", index)
        thread.join()
        # logging.info("Main    : thread %d done", index)

    end = timer()
    elapsed = end - start
    logging.info(f"Main    : all threads done in {timedelta(seconds=elapsed)}")

    return
    

if __name__ == "__main__":
    main()
    
