jaeger:
  sudo docker run -d -p6831:6831/udp -p6832:6832/udp -p16686:16686 -p14268:14268 jaegertracing/all-in-one:latest

build:
    # export RUSTFLAGS="-Awarnings"
    cargo build -r --bin ureq_threads

benchmark:
    bash ./benchmark.sh flask ureq_threads
  
  


