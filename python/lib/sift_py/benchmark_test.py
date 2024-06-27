from collections import deque
from typing import Dict, List, Tuple
from pytest_benchmark.fixture import BenchmarkFixture

def test_list_writes(benchmark: BenchmarkFixture):
    def write_to_list() -> int:
        res = []
        for _ in range(1_000_000):
            res.append(1_000_000)
        return len(res)

    assert benchmark(write_to_list) == 1_000_000

def test_deque_writes(benchmark: BenchmarkFixture):
    def write_to_deque() -> int:
        res = deque([], 1_000_000)
        for _ in range(1_000_000):
            res.append(1_000_000)
        return len(res)

    assert benchmark(write_to_deque) == 1_000_000

def test_clear(benchmark: BenchmarkFixture):
    def setup() -> Tuple[List[List[int]], Dict]:
        res = []
        for _ in range(1_000_000):
            res.append(1_000_000)
        return [res], {}

    def clear(subject: List[int]) -> List[int]:
        subject.clear()
        return subject

    res = benchmark.pedantic(clear, setup=setup)
    assert len(res) == 0

def test_re_init(benchmark: BenchmarkFixture):
    def setup() -> Tuple[List[List[int]], Dict]:
        res = []
        for _ in range(1_000_000):
            res.append(1_000_000)
        return [res], {}

    def clear(subject: List[int]) -> List[int]:
        subject = []
        return subject

    res = benchmark.pedantic(clear, setup=setup)
    assert len(res) == 0
