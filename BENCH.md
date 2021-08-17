See GitHub benchmark info on [workflow file](.github/workflows/bench.yaml)

# Machine info
```yaml
OS: Ubuntu 20.04.2 LTS x86_64
hyperfine: v1.11.0
gwc: 8.32
cpu: Intel(R) Xeon(R) Platinum 8272CL CPU @ 2.60GHz
```

# Summary
Benchmark shows:
- Byte count: gwc 1.84 ± 0.13 times faster than cw
- Line count: gwc 1.68 ± 0.09 times faster than cw
- **Word count: cw 2.07 ± 0.08 times faster than gwc**
- **Char count: cw 2.76 ± 0.03 times faster than gwc**
- **Max length: cw 2.32 ± 0.04 times faster than gwc**
- **Default mode: cw 1.72 ± 0.02 times faster than gwc**
- **High load: cw 1.77 ± 0.01 times faster than gwc**


# Results from workflow
```text
Byte count
Benchmark #1: cw -b Gabriel.txt Lorem_big.txt arabic.txt empty.txt french.txt sample1.txt sample2.txt sample3.txt small.txt spanish.txt weird.txt world192.txt
  Time (mean ± σ):       1.5 ms ±   0.1 ms    [User: 1.2 ms, System: 0.3 ms]
  Range (min … max):     1.4 ms …   2.2 ms    1474 runs
 
Benchmark #2: wc -c Gabriel.txt Lorem_big.txt arabic.txt empty.txt french.txt sample1.txt sample2.txt sample3.txt small.txt spanish.txt weird.txt world192.txt
  Time (mean ± σ):       0.8 ms ±   0.0 ms    [User: 0.7 ms, System: 0.1 ms]
  Range (min … max):     0.8 ms …   1.4 ms    2170 runs
 
Summary
  'wc -c Gabriel.txt Lorem_big.txt arabic.txt empty.txt french.txt sample1.txt sample2.txt sample3.txt small.txt spanish.txt weird.txt world192.txt' ran
    1.84 ± 0.13 times faster than 'cw -b Gabriel.txt Lorem_big.txt arabic.txt empty.txt french.txt sample1.txt sample2.txt sample3.txt small.txt spanish.txt weird.txt world192.txt'


Word count
Benchmark #1: cw -w Gabriel.txt Lorem_big.txt arabic.txt empty.txt french.txt sample1.txt sample2.txt sample3.txt small.txt spanish.txt weird.txt world192.txt
  Time (mean ± σ):      10.5 ms ±   0.4 ms    [User: 9.6 ms, System: 0.9 ms]
  Range (min … max):    10.4 ms …  17.3 ms    271 runs
 
Benchmark #2: wc -w Gabriel.txt Lorem_big.txt arabic.txt empty.txt french.txt sample1.txt sample2.txt sample3.txt small.txt spanish.txt weird.txt world192.txt
  Time (mean ± σ):      21.8 ms ±   0.1 ms    [User: 21.3 ms, System: 0.6 ms]
  Range (min … max):    21.6 ms …  22.3 ms    134 runs
 
Summary
  'cw -w Gabriel.txt Lorem_big.txt arabic.txt empty.txt french.txt sample1.txt sample2.txt sample3.txt small.txt spanish.txt weird.txt world192.txt' ran
    2.07 ± 0.08 times faster than 'wc -w Gabriel.txt Lorem_big.txt arabic.txt empty.txt french.txt sample1.txt sample2.txt sample3.txt small.txt spanish.txt weird.txt world192.txt'


Char count
Benchmark #1: cw -c Gabriel.txt Lorem_big.txt arabic.txt empty.txt french.txt sample1.txt sample2.txt sample3.txt small.txt spanish.txt weird.txt world192.txt
  Time (mean ± σ):       7.9 ms ±   0.1 ms    [User: 7.2 ms, System: 0.7 ms]
  Range (min … max):     7.8 ms …   8.5 ms    356 runs
 
Benchmark #2: wc -m Gabriel.txt Lorem_big.txt arabic.txt empty.txt french.txt sample1.txt sample2.txt sample3.txt small.txt spanish.txt weird.txt world192.txt
  Time (mean ± σ):      21.8 ms ±   0.2 ms    [User: 21.1 ms, System: 0.7 ms]
  Range (min … max):    21.7 ms …  23.0 ms    129 runs
 
Summary
  'cw -c Gabriel.txt Lorem_big.txt arabic.txt empty.txt french.txt sample1.txt sample2.txt sample3.txt small.txt spanish.txt weird.txt world192.txt' ran
    2.76 ± 0.03 times faster than 'wc -m Gabriel.txt Lorem_big.txt arabic.txt empty.txt french.txt sample1.txt sample2.txt sample3.txt small.txt spanish.txt weird.txt world192.txt'


Line count
Benchmark #1: cw -l Gabriel.txt Lorem_big.txt arabic.txt empty.txt french.txt sample1.txt sample2.txt sample3.txt small.txt spanish.txt weird.txt world192.txt
  Time (mean ± σ):       3.6 ms ±   0.1 ms    [User: 3.0 ms, System: 0.6 ms]
  Range (min … max):     3.5 ms …   4.4 ms    686 runs
 
Benchmark #2: wc -l Gabriel.txt Lorem_big.txt arabic.txt empty.txt french.txt sample1.txt sample2.txt sample3.txt small.txt spanish.txt weird.txt world192.txt
  Time (mean ± σ):       2.2 ms ±   0.1 ms    [User: 1.8 ms, System: 0.4 ms]
  Range (min … max):     2.0 ms …   4.5 ms    1125 runs
 
Summary
  'wc -l Gabriel.txt Lorem_big.txt arabic.txt empty.txt french.txt sample1.txt sample2.txt sample3.txt small.txt spanish.txt weird.txt world192.txt' ran
    1.68 ± 0.09 times faster than 'cw -l Gabriel.txt Lorem_big.txt arabic.txt empty.txt french.txt sample1.txt sample2.txt sample3.txt small.txt spanish.txt weird.txt world192.txt'


max length
Benchmark #1: cw -L Gabriel.txt Lorem_big.txt arabic.txt empty.txt french.txt sample1.txt sample2.txt sample3.txt small.txt spanish.txt weird.txt world192.txt
  Time (mean ± σ):       9.4 ms ±   0.1 ms    [User: 8.8 ms, System: 0.7 ms]
  Range (min … max):     9.3 ms …  10.5 ms    302 runs
 
Benchmark #2: wc -L Gabriel.txt Lorem_big.txt arabic.txt empty.txt french.txt sample1.txt sample2.txt sample3.txt small.txt spanish.txt weird.txt world192.txt
  Time (mean ± σ):      21.9 ms ±   0.3 ms    [User: 21.0 ms, System: 0.8 ms]
  Range (min … max):    21.6 ms …  24.7 ms    134 runs
 
Summary
  'cw -L Gabriel.txt Lorem_big.txt arabic.txt empty.txt french.txt sample1.txt sample2.txt sample3.txt small.txt spanish.txt weird.txt world192.txt' ran
    2.32 ± 0.04 times faster than 'wc -L Gabriel.txt Lorem_big.txt arabic.txt empty.txt french.txt sample1.txt sample2.txt sample3.txt small.txt spanish.txt weird.txt world192.txt'


Default mode
Benchmark #1: cw Gabriel.txt Lorem_big.txt arabic.txt empty.txt french.txt sample1.txt sample2.txt sample3.txt small.txt spanish.txt weird.txt world192.txt
  Time (mean ± σ):      12.7 ms ±   0.1 ms    [User: 11.9 ms, System: 0.8 ms]
  Range (min … max):    12.5 ms …  13.8 ms    228 runs
 
Benchmark #2: wc Gabriel.txt Lorem_big.txt arabic.txt empty.txt french.txt sample1.txt sample2.txt sample3.txt small.txt spanish.txt weird.txt world192.txt
  Time (mean ± σ):      21.8 ms ±   0.1 ms    [User: 21.0 ms, System: 0.7 ms]
  Range (min … max):    21.7 ms …  22.5 ms    132 runs
 
Summary
  'cw Gabriel.txt Lorem_big.txt arabic.txt empty.txt french.txt sample1.txt sample2.txt sample3.txt small.txt spanish.txt weird.txt world192.txt' ran
    1.72 ± 0.02 times faster than 'wc Gabriel.txt Lorem_big.txt arabic.txt empty.txt french.txt sample1.txt sample2.txt sample3.txt small.txt spanish.txt weird.txt world192.txt'


High load
Benchmark #1: cw Gabriel.txt Lorem_big.txt arabic.txt empty.txt french.txt sample1.txt sample2.txt sample3.txt small.txt spanish.txt weird.txt world192.txt Gabriel.txt Lorem_big.txt arabic.txt empty.txt french.txt sample1.txt sample2.txt sample3.txt small.txt spanish.txt weird.txt world192.txt Gabriel.txt Lorem_big.txt arabic.txt empty.txt french.txt sample1.txt sample2.txt sample3.txt small.txt spanish.txt weird.txt world192.txt
  Time (mean ± σ):      35.8 ms ±   0.1 ms    [User: 33.9 ms, System: 1.9 ms]
  Range (min … max):    35.6 ms …  36.4 ms    100 runs
 
Benchmark #2: wc Gabriel.txt Lorem_big.txt arabic.txt empty.txt french.txt sample1.txt sample2.txt sample3.txt small.txt spanish.txt weird.txt world192.txt Gabriel.txt Lorem_big.txt arabic.txt empty.txt french.txt sample1.txt sample2.txt sample3.txt small.txt spanish.txt weird.txt world192.txt Gabriel.txt Lorem_big.txt arabic.txt empty.txt french.txt sample1.txt sample2.txt sample3.txt small.txt spanish.txt weird.txt world192.txt
  Time (mean ± σ):      63.5 ms ±   0.2 ms    [User: 62.1 ms, System: 1.4 ms]
  Range (min … max):    63.2 ms …  64.9 ms    100 runs
 
Summary
  'cw Gabriel.txt Lorem_big.txt arabic.txt empty.txt french.txt sample1.txt sample2.txt sample3.txt small.txt spanish.txt weird.txt world192.txt Gabriel.txt Lorem_big.txt arabic.txt empty.txt french.txt sample1.txt sample2.txt sample3.txt small.txt spanish.txt weird.txt world192.txt Gabriel.txt Lorem_big.txt arabic.txt empty.txt french.txt sample1.txt sample2.txt sample3.txt small.txt spanish.txt weird.txt world192.txt' ran
    1.77 ± 0.01 times faster than 'wc Gabriel.txt Lorem_big.txt arabic.txt empty.txt french.txt sample1.txt sample2.txt sample3.txt small.txt spanish.txt weird.txt world192.txt Gabriel.txt Lorem_big.txt arabic.txt empty.txt french.txt sample1.txt sample2.txt sample3.txt small.txt spanish.txt weird.txt world192.txt Gabriel.txt Lorem_big.txt arabic.txt empty.txt french.txt sample1.txt sample2.txt sample3.txt small.txt spanish.txt weird.txt world192.txt'
```