# Buffered Ingestion Benchmark

The purpose of this benchmark is to provide an overall assessment of end-to-end ingestion performance using the sift-stack-py library, comparing both the buffered
and unbuffered ingestion client APIs. Each test is set up by parsing a CSV file containing 100K rows (not included in repo), with each row transformed into a single data ingestion request
consisting of a timestamp and 50 channel values. Once 100K requests are generated, the benchmark measures the time required to ingest all 100K requests in various batch
sizes into a local instance of Sift.

Since the test uses a loopback interface for ingesting data into a local instance of Sift, network bandwidth and the state of Sift's deployed servers is not a factor,
allowing the library's raw performance to be assessed. Furthermore, the unbuffered ingestion benchmark is designed to process requests in batches, similar to buffered ingestion,
but without the overhead of thread synchronization mechanisms.

Below are the benchmark results for different buffer and batch sizes.

## Benchmarks

Batch size: 1000
```
platform darwin -- Python 3.8.19, pytest-8.3.2, pluggy-1.5.0
benchmark: 4.0.0 (defaults: timer=time.perf_counter disable_gc=False min_rounds=5 min_time=0.000005 max_time=1.0 calibration_precision=10 warmup=False warmup_iterations=100000)
rootdir: /Users/benjaminnguyen/Code/sift/python
configfile: pyproject.toml
plugins: benchmark-4.0.0
collected 2 items                                                                                                                                                                                

test_ingestion_performance.py
unbuffered_ingestion | num_flows=100000 batch_size=1000 avg_time_per_batch=0.05139007806777954s
unbuffered_ingestion | num_flows=100000 batch_size=1000 avg_time_per_batch=0.0486946439743042s
unbuffered_ingestion | num_flows=100000 batch_size=1000 avg_time_per_batch=0.05157544374465942s
unbuffered_ingestion | num_flows=100000 batch_size=1000 avg_time_per_batch=0.051327617168426515s
unbuffered_ingestion | num_flows=100000 batch_size=1000 avg_time_per_batch=0.04842857360839844s


--------------------------------------------------------------------------------------- benchmark: 2 tests ---------------------------------------------------------------------------------------
Name (time in s)                           Min               Max              Mean            StdDev            Median               IQR            Outliers     OPS            Rounds  Iterations
--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_ingestion_performance              4.8429 (1.0)      5.1577 (1.0)      5.0284 (1.0)      0.1577 (1.0)      5.1329 (1.0)      0.2808 (1.0)           2;0  0.1989 (1.0)           5           1
test_buffered_ingestion_performance     4.9934 (1.03)     6.0979 (1.18)     5.5124 (1.10)     0.4864 (3.08)     5.3261 (1.04)     0.8497 (3.03)          2;0  0.1814 (0.91)          5           1
--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

Legend:
  Outliers: 1 Standard Deviation from Mean; 1.5 IQR (InterQuartile Range) from 1st Quartile and 3rd Quartile.
  OPS: Operations Per Second, computed as 1 / Mean
```

Batch size: 2500
```
platform darwin -- Python 3.8.19, pytest-8.3.2, pluggy-1.5.0
benchmark: 4.0.0 (defaults: timer=time.perf_counter disable_gc=False min_rounds=5 min_time=0.000005 max_time=1.0 calibration_precision=10 warmup=False warmup_iterations=100000)
rootdir: /Users/benjaminnguyen/Code/sift/python
configfile: pyproject.toml
plugins: benchmark-4.0.0
collected 2 items                                                                                                                                                                                

test_ingestion_performance.py .unbuffered_ingestion | num_flows=100000 batch_size=2500 avg_time_per_batch=0.13309289813041686s
unbuffered_ingestion | num_flows=100000 batch_size=2500 avg_time_per_batch=0.13569029569625854s
unbuffered_ingestion | num_flows=100000 batch_size=2500 avg_time_per_batch=0.14162625670433043s
unbuffered_ingestion | num_flows=100000 batch_size=2500 avg_time_per_batch=0.1334519863128662s
unbuffered_ingestion | num_flows=100000 batch_size=2500 avg_time_per_batch=0.15238019824028015s
.


--------------------------------------------------------------------------------------- benchmark: 2 tests ---------------------------------------------------------------------------------------
Name (time in s)                           Min               Max              Mean            StdDev            Median               IQR            Outliers     OPS            Rounds  Iterations
--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_ingestion_performance              5.3238 (1.0)      6.0953 (1.0)      5.5700 (1.0)      0.3239 (1.01)     5.4277 (1.0)      0.4381 (1.0)           1;0  0.1795 (1.0)           5           1
test_buffered_ingestion_performance     5.4040 (1.02)     6.1245 (1.00)     5.8257 (1.05)     0.3208 (1.0)      5.9095 (1.09)     0.5724 (1.31)          1;0  0.1717 (0.96)          5           1
--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

Legend:
  Outliers: 1 Standard Deviation from Mean; 1.5 IQR (InterQuartile Range) from 1st Quartile and 3rd Quartile.
  OPS: Operations Per Second, computed as 1 / Mean
```

Batch size: 5000
```
platform darwin -- Python 3.8.19, pytest-8.3.2, pluggy-1.5.0
benchmark: 4.0.0 (defaults: timer=time.perf_counter disable_gc=False min_rounds=5 min_time=0.000005 max_time=1.0 calibration_precision=10 warmup=False warmup_iterations=100000)
rootdir: /Users/benjaminnguyen/Code/sift/python
configfile: pyproject.toml
plugins: benchmark-4.0.0
collected 2 items                                                                                                                                                                                

test_ingestion_performance.py .unbuffered_ingestion | num_flows=100000 batch_size=5000 avg_time_per_batch=0.26795496940612795s
unbuffered_ingestion | num_flows=100000 batch_size=5000 avg_time_per_batch=0.28187270164489747s
unbuffered_ingestion | num_flows=100000 batch_size=5000 avg_time_per_batch=0.27908159494400026s
unbuffered_ingestion | num_flows=100000 batch_size=5000 avg_time_per_batch=0.31943129301071166s
unbuffered_ingestion | num_flows=100000 batch_size=5000 avg_time_per_batch=0.2647546291351318s
.


--------------------------------------------------------------------------------------- benchmark: 2 tests ---------------------------------------------------------------------------------------
Name (time in s)                           Min               Max              Mean            StdDev            Median               IQR            Outliers     OPS            Rounds  Iterations
--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_buffered_ingestion_performance     5.0722 (1.0)      5.5800 (1.0)      5.2959 (1.0)      0.1979 (1.0)      5.3069 (1.0)      0.2887 (1.0)           2;0  0.1888 (1.0)           5           1
test_ingestion_performance              5.2951 (1.04)     6.3887 (1.14)     5.6524 (1.07)     0.4362 (2.20)     5.5817 (1.05)     0.4822 (1.67)          1;0  0.1769 (0.94)          5           1
--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

Legend:
  Outliers: 1 Standard Deviation from Mean; 1.5 IQR (InterQuartile Range) from 1st Quartile and 3rd Quartile.
  OPS: Operations Per Second, computed as 1 / Mean
```

Batch size: 10000
```
platform darwin -- Python 3.8.19, pytest-8.3.2, pluggy-1.5.0
benchmark: 4.0.0 (defaults: timer=time.perf_counter disable_gc=False min_rounds=5 min_time=0.000005 max_time=1.0 calibration_precision=10 warmup=False warmup_iterations=100000)
rootdir: /Users/benjaminnguyen/Code/sift/python
configfile: pyproject.toml
plugins: benchmark-4.0.0
collected 2 items                                                                                                                                                                                

test_ingestion_performance.py .unbuffered_ingestion | num_flows=100000 batch_size=10000 avg_time_per_batch=0.642977499961853s
unbuffered_ingestion | num_flows=100000 batch_size=10000 avg_time_per_batch=0.5820534706115723s
unbuffered_ingestion | num_flows=100000 batch_size=10000 avg_time_per_batch=0.5236365318298339s
unbuffered_ingestion | num_flows=100000 batch_size=10000 avg_time_per_batch=0.4863920211791992s
unbuffered_ingestion | num_flows=100000 batch_size=10000 avg_time_per_batch=0.5768257141113281s
.


--------------------------------------------------------------------------------------- benchmark: 2 tests ---------------------------------------------------------------------------------------
Name (time in s)                           Min               Max              Mean            StdDev            Median               IQR            Outliers     OPS            Rounds  Iterations
--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_ingestion_performance              4.8639 (1.0)      6.4298 (1.06)     5.6238 (1.0)      0.5993 (1.95)     5.7683 (1.00)     0.8296 (1.85)          2;0  0.1778 (1.0)           5           1
test_buffered_ingestion_performance     5.3255 (1.09)     6.0900 (1.0)      5.7817 (1.03)     0.3074 (1.0)      5.7551 (1.0)      0.4486 (1.0)           2;0  0.1730 (0.97)          5           1
--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

Legend:
  Outliers: 1 Standard Deviation from Mean; 1.5 IQR (InterQuartile Range) from 1st Quartile and 3rd Quartile.
  OPS: Operations Per Second, computed as 1 / Mean
```

## Conclusions

The following are the key takeaways from the benchmarks:

- **Ingestion Speed**: Ingesting 1000 requests, each containing 50 double-type channel values, to a local Sift instance takes an average of 0.05 seconds. This translates to a 20 kHz ingestion rate. Note that this does not account for network bandwidth or the performance of Sift's deployed servers.
- **Buffered vs. Manual Batching**: Buffered ingestion is slightly slower than manual batching, but it significantly reduces the setup effort. Overall, buffered ingestion is recommended for simplicity and efficiency.
- **Batch Size Considerations**: Increasing the batch or buffer size doesn't necessarily improve raw ingestion performance. However, it is important to note that larger batches take longer to serialize, which can cause a thread to hold the Global Interpreter Lock (GIL) for extended periods. This is something to consider in multi-threaded CPython applications.
