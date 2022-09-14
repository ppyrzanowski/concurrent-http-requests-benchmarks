# async_vs_threads (more like benchmarking performance of concurrent requests with dedicated `reqwest` http client)

## Results

```
(venv) ~/Dev/Rust/async_vs_threads/python-client (main ✗) python client.py
22:55:06.379: Main    : creating 500 threads
22:55:06.937: Main    : done creating threads
22:55:08.041: Main    : all threads done in 0:00:01.661702
(venv) ~/Dev/Rust/async_vs_threads/python-client (main ✗)
```

## TODO

- Add task performance export option as csv
- Parse/Plot exported task performance with python or excel
- Create small blog-post or writeup

https://crates.io/crates/opentelemetry
https://crates.io/crates/tracing-timing


## SYSTEM CHANGES FOR FLAMEGRAPH PROFILING
> For my own sanity

### Command to start profiling
Make sure to run rust with release profile and turn on `debug` option
```sh
~/Dev/Rust/async_vs_threads (main ✗) cargo flamegraph --example async_request_tasks -- 4
```

### CHANGES TO SYSTEM/KERNEL MADE:
#### 1.

> VALUE WAS 4 
> CHANGED TO -1 FOR MONITORING CPU INSTRUCTIONS/PROFILEING WITH `perf`

```sh
sudo sh -c 'echo kernel.perf_event_paranoid=1 > /etc/sysctl.d/local.conf'
```


#### 2.
```sh
~/Dev/Rust/async_vs_threads (main ✗) cat /proc/sys/kernel/kptr_restrict
1
~/Dev/Rust/async_vs_threads (main ✗) echo 0 > /proc/sys/kernel/kptr_restrict
zsh: permission denied: /proc/sys/kernel/kptr_restrict
~/Dev/Rust/async_vs_threads (main ✗) sudo bash
root@patrik-ThinkPad-T15-Gen-1:/home/patrik/Dev/Rust/async_vs_threads# echo 0 > /proc/sys/kernel/kptr_restrict
root@patrik-ThinkPad-T15-Gen-1:/home/patrik/Dev/Rust/async_vs_threads# exit
exit
```


### Sources

https://blog.logrocket.com/an-introduction-to-profiling-a-rust-web-application/

https://github.com/flamegraph-rs/flamegraph
