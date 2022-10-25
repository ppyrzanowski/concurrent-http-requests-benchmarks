# concurrent_http_requests_benchmarks 

Benchmarking performance of x concurrent requests sent per second by given implementation.




## TODO

- Create small blog-post or writeup


---

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


## Sources

https://crates.io/crates/opentelemetry
https://crates.io/crates/tracing-timing

https://blog.logrocket.com/an-introduction-to-profiling-a-rust-web-application/

https://github.com/flamegraph-rs/flamegraph
