#!/bin/bash


# ----------------
# SPECIAL HANDLERS
# ----------------

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


# -----
# CLI utils
# -----

# Shell seperator for better readability
seperator_line() {
  printf "%s\n" "--------------------------------------------------------------"
}

# Prettyprint benchmark title to shell
print_benchmark_title() {
  local A=$(printf "%0.1s" " "{1..50});
  A="[Benchmark]${A}"
  local B="$CLIENT_IMPL"

  printf "${A:0:-${#B}} $B\n"
}


# -----------
# Output handler
# -----------

# Writes the results to a file.
write_result() {
  printf "Written output to ${BENCHMARK_RESULTS_FILEPATH}\n"
  printf "$OUTPUT\n\n" >> "$BENCHMARK_RESULTS_FILEPATH"
}


# -----------------------
# Server-runner functions
# -----------------------

# Starts the server receiving our requests, the requests should be handled 
# concurrently as well (although this is implementaion detail about the server which we do not care about).
start_server() {
  case "$SERVER_IMPL" in
  "python-flask")
    . ./python-server-flask/env/bin/activate
    mkdir -p ./python-server-flask/logs
    # Redirect logs of flask app
    flask --app ./python-server-flask/server run >>./python-server-flask/logs/$(date -d "today" +"%Y%m%d%H%M").log 2>&1 & SERVER_PID=$! 
    # Wait for server to start in background
    sleep 3
    deactivate
    ;;
  "python-async")
    printf "Python asyncio server script branch not implemented yet\n"
    exit 1
    ;;
  *)
    # Restore default exit handler
    trap EXIT
    printf "Invalid backend implementation option (${SERVER_IMPL}).\n"
    exit 1
    ;;
  esac
}

# Stop the server after benchmarks are done.
shutdown_server() {
  if [[ -n $SERVER_PID ]]; then
    kill "$SERVER_PID"
    printf "Shutdown server done.\n"
  fi
}


# ----------------------
# Client-runnerfunctions
# ----------------------

# Starts the client, sending x requests as fast as possible by given implementaion to our server.
start_client() {
  local EXECUTION_TIME=0
  case "$CLIENT_IMPL" in
  "ureq_threads")
    # Compile client once
    if [[ $CLIENT_COMPILED -lt 1 ]]; then
      printf "(Compiling client...)\n"
      export RUSTFLAGS="$RUSTFLAGS -Awarnings"
      cargo build -r --manifest-path ./rust-client/Cargo.toml
      CLIENT_COMPILED=1
    fi
    EXECUTION_TIME=$( ./rust-client/target/release/rust_request_clients threads-ureq $NUM_OF_TASKS ) 
    ;;
  "python")
    # trap "clean_up_python; interrupt_handler" "INT"
    . ./python-client/env/bin/activate
    EXECUTION_TIME=$( python3 ./python-client/client.py $NUM_OF_TASKS )
    ;;
  *)
    printf "Please provide client type (ureq_threads | python)\n"
    exit 1
    ;;
  esac 

  OUTPUT=$(printf "${OUTPUT}${EXECUTION_TIME},")
  printf "Executed %04s request(s) in %04dms\n" $NUM_OF_TASKS $EXECUTION_TIME
}


# -------------------
# Benchmark functions
# -------------------

# Creates a single CSV row
benchmark() {
  print_benchmark_title

  # Row-header, row-implementaion-type
  OUTPUT="${OUTPUT}${CLIENT_IMPL},"
  NUM_OF_TASKS=$TASKS_BASE_COUNT

  local BENCHMARK=0
  while [ $BENCHMARK -lt $(($NUM_OF_BENCHMARKS)) ]
  do
    ((BENCHMARK++))

    if [[ $NUM_OF_TASKS -gt $MAX_REQUESTS ]]
    then
      printf "Number of tasks ($NUM_OF_TASKS) exceeds max allowed number of reqeusts ($MAX_REQUESTS)!\n"
      break
    fi

    start_client
    ((NUM_OF_TASKS*=2))
  done 

  # End of row
  OUTPUT="${OUTPUT}\n"

  seperator_line
}

# Executes all predefined benchmarks
default_benchmarks() {
  # Interruption handlers
  # fires on `kill $$` and `exit`
  trap exit_handler EXIT
  trap interrupt_handler INT

  local CLI_MESSAGE="\nBenchmarking number of concurrent requests sent per second.\n\n"
  CLI_MESSAGE="${CLI_MESSAGE}Running default benchmarks.\n"
  printf "$CLI_MESSAGE"
  seperator_line

  # Start server in background process
  SERVER_IMPL="python-flask"
  start_server

  # Run Benchmarks against the server
  local SAMPLE=0
  while [[ $SAMPLE -lt $NUM_OF_SAMPLES ]]
  do
    for IMPL in "ureq_threads" "python"
    do 
      CLIENT_IMPL=$IMPL 
      benchmark
    done
    ((SAMPLE = SAMPLE + 1))
  done
  exit 0
}


# ---------------------------
# BENCHMARK SCRIPT ENTRYPOINT
# ---------------------------

# An implementation:
# - must print the total execution time to `stdout` in milliseconds as integer.
# - should only take one CLI argument, defining the amount of requests (threads/tasks) to execute concurrently.
# - should write logs to their own logger/filehandler located at 
#   `./client-<impl.>/logs/$(date -d "today" +"%Y%m%d%H%M").log`.


# Configuration variables
MAX_REQUESTS=${MAX_REQUESTS:=1200}        # For safety, max allowed number of requests to try send at once.
NUM_OF_SAMPLES=${NUM_OF_SAMPLES:=2}       # Number of benchmark-cycle repetitions for avarage calculation.
NUM_OF_BENCHMARKS=${NUM_OF_BENCHMARKS:=2} # Number of columns with doubling number of task per column (Factor).
TASKS_BASE_COUNT=${TASKS_BASE_COUNT:=1}   # Start number of requests to send by client ( x <= $MAX_REQUESTS ).
CLIENT_COMPILED=${CLIENT_COMPILED:=0}     # If the rust client was already compiled. Switch to 1 if no compilation is needed.

# Script global variables
OUTPUT=""                                 # Holds the benchmark results seperated by comma (CSV).

# Benchmark results path
mkdir -p benchmarks
BENCHMARK_RESULTS_FILEPATH="benchmarks/$(date -d "today" +"%Y%m%d%H%M").csv"
touch "$BENCHMARK_RESULTS_FILEPATH"

default_benchmarks

