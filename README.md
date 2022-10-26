# Benchmarking different implementations of SHA256 CBC

The testing has been performed on Windows 7x64 host with Ubuntu 18.04 x64 virtual machine with 8Gb RAM and 4 cores, Intel-VT enabled.

Host:
- CPU: Intel Core i7-4790 3.6 GHz 4 cores / 8 threads
- RAM: 32 Gb

AVX2 is used whenever possible.

Full release optimizations.

Encrypt/Decrypt is used over 1024 bytes data with 1'000'000 cycles.

All tests use 1 core.

Test results are given in milliseconds of total execution time (encrypt/decrypt), in descending order, worst to best.

You can find links to the algorithms' web pages in the first comments of the corresponding main source files.

## Test Results with SHA-256

| Language | Author  | Time, ms  |
|:-------:|:---------:|:---------:|
|  rust  | DaGenix   | 3771/2795 |
|  rust  | RustCrypto   | 2046/960 |
|  rust  | openssl   | 2023/513 |