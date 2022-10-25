jaeger:
  sudo docker run -d -p6831:6831/udp -p6832:6832/udp -p16686:16686 -p14268:14268 jaegertracing/all-in-one:latest

install-python-env:
  #!/usr/bin/env bash
  cd ./python-client
  python3 -m venv env
  . ./env/bin/activate
  pip install -r requirements.txt
  deactivate
  cd ..

  cd ./python-server-flask/
  python3 -m venv env
  . ./env/bin/activate
  pip install -r requirements.txt
  deactivate

py-server:
  #!/usr/bin/env bash
  . ./python-server-flask/env/bin/activate
  flask --app python-server-flask/server run &
  deactivate

py-server-kill:
  pkill -e flask

benchmark:
    bash ./benchmark.sh

py-client num_requests: 
  #!/usr/bin/env bash
  . ./python-client/env/bin/activate
  python3 python-client/client.py {{num_requests}}
  deactivate

