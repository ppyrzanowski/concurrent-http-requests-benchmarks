#!/bin/bash

clean_up () {
  shutdown_server
  write_result
  printf "Done\n"
}

# Executed before `exit` terminates
exit_handler () {
  printf "Cleaning up and writing results...\n"
  clean_up
}

# Handle Ctrl-C (SIGINT) signal
interrupt_handler () {
  trap SIGINT # Restore signal handling for SIGINT
  printf "\n"
  seperator_line
  printf "Benchmarks interrupted.\n"
  exit 1
}

# Shell seperator for better readability
seperator_line() {
  printf "%s\n" "--------------------------------------------------------------"
}

# Prettyprint benchmark title to shell
print_benchmark_title() {
  A=$(printf "%0.1s" " "{1..50});
  A="[Benchmark]${A}"
  B="$CLIENT_IMPL"

  echo "${A:0:-${#B}} $B"
}

# Writes the results to a file.
write_result() {
    printf "Written output to ${benchmark_results_path}\n"
    printf "$output\n" >> "$benchmark_results_path"
}

# Starts the server receiving our requests, the requests should be handled 
# concurrently as well (although this is implementaion detail which we do 
# not care about).
start_server() {
  case "$1" in
  # flask = python flask server
  "flask")
    . ./python-server-flask/venv/bin/activate
    mkdir -p logs
    # Redirect logs of flask app
    flask --app ./python-server-flask/server run >>./python-server-flask/logs/$(date -d "today" +"%Y%m%d%H%M").log 2>&1 & SERVER_PID=$! 
    # Wait for server to start in background
    sleep 3
    deactivate
    ;;
  # async = python asyncio server
  "async")
    printf "Python asyncio server script branch not implemented yet\n"
    exit 1
    ;;
  *)
    printf "Please provide backend type (flask | async)\n"
    exit 1
    ;;
  esac
}

# Starts the client, sending x requests as fast as possible by given implementaion to our server.
start_client() {
  case "$1" in
  "ureq_threads")
    # Compile client once
    if [[ $CLIENT_COMPILED -lt 1 ]]; then
      printf "(Compiling client...)\n"
      export RUSTFLAGS="$RUSTFLAGS -Awarnings"
      cargo build -r --bin ureq_threads
      CLIENT_COMPILED=1
    fi
    execution_time=$( ./target/release/ureq_threads $NUM_OF_TASKS ) 
    ;;
  "python")
    # trap "clean_up_python; interrupt_handler" "INT"
    . ./python-client/venv/bin/activate
    execution_time=$( python ./python-client/client.py $NUM_OF_TASKS )
    ;;
  *)
    printf "Please provide client type (ureq_threads | python)\n"
    exit 1
    ;;
  esac 

  output=$(printf "${output}${execution_time},")
  printf "Executed %04s request(s) in %04dms\n" $NUM_OF_TASKS $execution_time
}

# Stop the server after benchmarks are done.
shutdown_server() {
  if [[ -n $SERVER_PID ]]; then
    kill "$SERVER_PID"
    printf "Shutdown server done.\n"
  fi
}

# Benchmarks a single row
benchmark() {
  print_benchmark_title

  # Row-header, row-implementaion-type
  output="${output}${CLIENT_IMPL},"
  NUM_OF_TASKS=1
  start_client "$CLIENT_IMPL"

  i=0
  while [ $i -lt $(($NUM_OF_BENCHMARKS-1)) ]
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

  # End of row
  output="${output}\n"

  seperator_line
}

# Executes all predefined benchmarks
default_benchmarks() {
  # fires on `kill $$` and `exit`
  trap exit_handler EXIT
  trap interrupt_handler INT

  cli_message="${cli_message}Running default benchmarks.\n"
  printf "$cli_message"
  seperator_line

  SERVER_IMPL="flask"
  start_server "$SERVER_IMPL"

  sample=0
  while [[ $sample -le $NUM_OF_SAMPLES ]]
  do
    for impl in "ureq_threads" "python"
    do 
      CLIENT_IMPL=$impl
      benchmark
    done
    ((sample = sample + 1))
  done
  exit 0
}


# Script entrypoint

export CLIENT_COMPILED=0  # 1 - If the rust client was already compiled by previous benchmark iteration
export SERVER_IMPL=""
export CLIENT_IMPL=""
NUM_OF_SAMPLES=2          # Number of benchmark-cycle repetitions for avarage calculation
NUM_OF_BENCHMARKS=2       # Number of columns with doubling number of task per column (Factor)
# NUM_OF_TASKS="${3:-1}"    # Number of requests to send by client (default: 1)
MAX_REQUESTS=2000         # For safety, max allowed number of requests to try send at once

# `output` holds the benchmark results seperated by comma (CSV)
output=""


cli_message="\nBenchmarking number of concurrent requests sent per second in Python VS Rust.\n\n"

# Benchmark results path
mkdir -p benchmarks
benchmark_results_path=benchmarks/$(date -d "today" +"%Y%m%d%H%M").csv
touch "$benchmark_results_path"

default_benchmarks
