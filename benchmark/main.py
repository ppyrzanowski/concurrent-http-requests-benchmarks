from pathlib import Path
from abc import ABC, abstractmethod
import subprocess
import time


class HttpServerImplementation(ABC):
    role = "server"

    def __init__(self, language: str, name: str):
        self.language = language
        self.name = name

    @abstractmethod
    def run(self):
        pass


class PythonServerFlask(HttpServerImplementation):
    pid: int | None = None

    def __init__(self):
        super().__init__(language="python", name="flask")

    def run(self):
        # subprocess.run(["source", "../python-server-flask/env/bin/activate"], stdout=subprocess.PIPE, text=True)
        # today_str = date.today().strftime('%Y_%m_%d_%H_%M')
        # logs_file_path = Path(f"./python-server-flask/logs/{today_str}.log")
        # logs_file_path.parent.mkdir(parents=True, exist_ok=True)
        # with logs_file_path.open("a") as log_file:
        #     subprocess.Popen(["flask", "--app", "./python-server-flask/server" "run"], stdout=log_file)

        if Path.cwd().name != "benchmark":
            raise ValueError("main.py must be called within benchmark directory.")

        out = subprocess.Popen(["./start_gunicorn.sh"], cwd="../python-server-flask/",stdout=subprocess.PIPE)
        if out.stdout is None:
            return

        self.pid = int(out.stdout.readline().decode("utf-8").strip())

    def terminate(self):
        subprocess.run(["kill", "-INT", f"{self.pid}"])


class HttpClientImplementation(ABC):
    role = "client"

    def __init__(self, name: str, language: str):
        self.name = name
        self.language = language

    @abstractmethod
    def before_run(self):
        raise NotImplemented

    @abstractmethod
    def after_run(self):
        raise NotImplemented

    @abstractmethod
    def run(self):
        pass


def write_results():
    # Benchmark results path
    results_fp = Path("../benchmarks/").mkdir(parents=True, exist_ok=True)

def main():
    flask_app = PythonServerFlask()
    flask_app.run()
    time.sleep(4)
    flask_app.terminate()


if __name__ == "__main__":
    main()

