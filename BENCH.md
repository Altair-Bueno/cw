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

To see the full benchmark output, check [stats.txt](bench/stats.txt)