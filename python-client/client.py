import requests
import threading
import logging
from timeit import default_timer as timer
from datetime import timedelta, datetime
import sys
from pathlib import Path

def thread_function(name):
    # logging.info("Thread %s: starting", name)

    # url = "http://167.235.133.26:80/delay/2"
    url = f"http://127.0.0.1:5000/sleep/{name}"
    ses = requests.session()

    res = ses.get(url)
    # logging.info(f"Thread %s: response {res.text}", name)


def main():
    t = int(sys.argv[1])
    # format = "%(asctime)s.%(msecs)03d: %(message)s"
    format = '%(asctime)s.%(msecs)d %(name)s %(levelname)s %(message)s'
    Path("./logs").mkdir(parents=True, exist_ok=True)
    filepath = f"logs/{datetime.now().strftime('%Y%m%d%H%M')}.log"
    logging.basicConfig(
        filename=filepath,
        filemode="a",
        format=format,
        level=logging.INFO,
        datefmt="%H:%M:%S"
    )

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
    elapsed = timedelta(seconds=end - start)
    elapsed_ms = int(elapsed.total_seconds() * 1000)

    logging.info(f"Main    : all threads done in {elapsed} ({elapsed_ms})")
    print(elapsed_ms)

    return
    

if __name__ == "__main__":
    main()
    
