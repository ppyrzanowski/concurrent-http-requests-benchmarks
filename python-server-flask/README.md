# python-server-flask

Runs a Flask app (WSGI application) on a Gunicorn (WSGI server) server.

## Installation
```sh
python3 -m venv env

pip install -r ./requirements.txt

# Simple run command.
flask --app server run
```

## Run Flaks with Gunicorn HTTP server

```sh
# Set script premission as executable
chmod +x start_gunicorn.sh

# Run gunicorn with configuration
sudo ./start_gunicorn.sh
```

