#!/bin/bash

seperator_line() {
  printf "%s\n" "---------------------------------------------------"
}

# Writes the results to a file.
# TODO: export as CSV format
write_result() {
    mkdir -p benchmarks
    filename=benchmarks/${CLIENT_IMPL}$(date -d "today" +"%Y%m%d%H%M").log
    printf "$output\n" > "$filename"

}

# Starts the server receiving our requests, the requests should be handled 
# concurrently as well (although this is implementaion detail which we do 
# not care about).
start_server() {
  case "$1" in
  # flask = python flask server
  "flask")
    cd ./python-server-flask/
    . ./venv/bin/activate
    mkdir -p logs
    # Redirect logs of flask app to ./logs/{timestamp}.log
    flask --app server run >>./logs/$(date -d "today" +"%Y%m%d%H%M").log 2>&1 & SERVER_PID=$! 
    # Wait for server to start in background
    sleep 3
    deactivate
    cd ./..
    ;;
  # async = python asyncio server
  "async")
    printf "Python asyncio server script branch not implemented yet\n"
    exit 1
    ;;
  *)
    printf "Please provide backend type (flask | async)\n"
    ;;
  esac
}

# Starts the client, sending x requests as fast as possible to our server.
start_client() {
  case "$1" in
  "ureq_threads")
    # Compile client once
    if [[ $CLIENT_COMPILED -lt 1 ]]; then
      printf "Compiling client...\n"
      export RUSTFLAGS="$RUSTFLAGS -Awarnings"
      cargo build -r --bin ureq_threads
      CLIENT_COMPILED=1
    fi
    execution_time=$( ./target/release/ureq_threads $NUM_OF_TASKS ) 
    ;;
  "python")
    cd ./python-client/
    . ./venv/bin/activate
    execution_time=$( python client.py $NUM_OF_TASKS )
    deactivate
    cd ./..
    ;;
  *)
    printf "Please provide client type (ureq_threads | python)\n"
    return 1
    ;;
  esac 

  output=$(printf "${output}${execution_time},")
  printf "Executed %04s request(s) in %04dms\n" $NUM_OF_TASKS $execution_time
}

# Stop the server after benchmarks are done.
shutdown_server() {
  kill "$SERVER_PID"
  printf "Shutdown server done."
}

# Benchmarks a single row
benchmark() {
  i=0
  NUM_OF_TASKS=1
  start_client "$CLIENT_IMPL"
  while [ $i -le "$NUM_OF_BENCHMARKS" ]
  do
    i=$(($i+1))

    if [[ $NUM_OF_TASKS -gt $MAX_REQUESTS ]]
    then
      printf "Number of tasks ($NUM_OF_TASKS) exceeds max allowed number of reqeusts ($MAX_REQUESTS)!"
      break
    fi

    NUM_OF_TASKS=$((${NUM_OF_TASKS}*2))
    start_client "$CLIENT_IMPL"
  done 
  seperator_line

  write_result
}

# TODO: Finish later
custom_benchmarks() {
  cli_message="${cli_message}Running custom benchmarks.\n"
  cli_message="${cli_message}Client: ${CLIENT_IMPL}\n"
  cli_message="${cli_message}Server: ${SERVER_IMPL}\n"
  printf "$cli_message"
  seperator_line

  start_server "$SERVER_IMPL"

  benchmark

  seperator_line

  shutdown_server

  write_result
}

default_benchmarks() {
  cli_message="${cli_message}Running default benchmarks.\n"
  printf "$cli_message"
  seperator_line

  SERVER_IMPL="flask"
  start_server "$SERVER_IMPL"

  # -------- RUST ----------
  CLIENT_IMPL="ureq_threads"
  benchmark

  # -------- Python --------
  CLIENT_IMPL="python"
  benchmark

  shutdown_server
}


# Script entrypoint

export CLIENT_COMPILED=0
export SERVER_IMPL="$1"
export CLIENT_IMPL="$2"
NUM_OF_BENCHMARKS=4     # Number of columns with doubling number of task per column
MAX_REQUESTS=2000       # For safety
NUM_OF_TASKS="${3:-1}"  # Number of requests to send by client (default: 1)

# `output` holds the benchmark results seperated by comma (single-line)
output=""


cli_message="Benchmarking number of concurrent requests sent per second in Python VS Rust.\n\n"

if [[ -z $SERVER_IMPL || -z $CLIENT_IMPL ]]; then
  default_benchmarks
else
  custom_benchmarks
fi
