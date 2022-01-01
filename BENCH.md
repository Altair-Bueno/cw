This file provides benchmark results against GNU's coreutils wc. These results
show a considerable speed increase on most scenarios. Note that both programs
run extremely fast, and results may vary on different CPU architectures. While

cw's blazing performance comes with zero abstraction cost it might hurt
performance on poorly supported platforms.

# Machine info

```yaml
OS: Ubuntu 20.04.2 LTS x86_64
hyperfine: v1.11.0
cw: 1.0.0
gwc: 8.30
cpu: Intel(R) Xeon(R) Platinum 8171M CPU @ 2.60GHz
```

- Full version info [here](bench/version.txt)
- Full CPU info [here](bench/version.txt)

# Summary

Benchmark shows:

- Byte count: gwc 1.77 ± 0.57 times faster than cw
- Line count: gwc 1.69 ± 0.34 times faster than cw
- **Word count: cw 2.07 ± 0.15 times faster than gwc**
- **Char count: cw 2.71 ± 0.26 times faster than gwc**
- **Max length: cw 2.36 ± 0.13 times faster than gwc**
- **Default mode: cw 1.74 ± 0.16 times faster than gwc**
- **High load: cw 1.76 ± 0.10 times faster than gwc**

To see the full benchmark output, check [stats.txt](bench/stats.txt)