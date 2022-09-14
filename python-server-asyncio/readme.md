# Server options for handling concurrent reqeusts from Rust

## Local python server
Credits to: [Making 1 million requests with python-aiohttp by Paweł Miech](https://pawelmhm.github.io/asyncio/python/aiohttp/2016/04/22/asyncio-aiohttp.html)

Simple async python server, responding on every route with [Frankenstein by Marry Shelley](./frank.html) as txt (no html markup added to html file)

Create python environment
```sh
# in current path create env ($PROJECT_ROOT/server/)
python3 -m venv .venv

# install dependencies required in main.py
pip install <name>
```

## Httpbin server on docker

[https://httpbin.org/](https://httpbin.org/)

```sh
docker run -p 80:80 kennethreitz/httpbin
```

### Running on:
Hetzner server: `docker-ce-ubuntu-4gb-nbg1-2`
Public ip: `http://167.235.133.26:80/`

