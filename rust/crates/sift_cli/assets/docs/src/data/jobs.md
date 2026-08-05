# Jobs

Server-side work in Sift — data imports, exports, and rule evaluations — runs
as a job. The `get`, `status`, and `wait` commands inspect and poll those jobs,
which is what makes the CLI usable in CI/CD.

## Why not just pass `--wait`?

`sift-cli import ... --wait` blocks until the job finishes. That is the right
default for interactive use, but it serializes work: firing five imports means
five sequential blocking commands. And it does not compose — you cannot start
several imports in parallel, do other work, then confirm they all landed at
the end.

The verb-first commands split those two concerns:

1. **Kick off work without waiting.** `sift-cli import ...` (no `--wait`)
   uploads the file and prints the assigned job ID. It exits `0` even if the
   server-side job later fails.
2. **Poll the jobs later.** `sift-cli wait job <ID> [ID ...]` blocks until
   every named job reaches a terminal state and exits non-zero if any failed.

## The CI/CD pattern

```sh
# Fire imports in parallel; capture each job id.
JOB_A=$(sift-cli import csv a.csv --asset engine | awk '/Job ID/ {print $NF}')
JOB_B=$(sift-cli import csv b.csv --asset engine | awk '/Job ID/ {print $NF}')
JOB_C=$(sift-cli import csv c.csv --asset engine | awk '/Job ID/ {print $NF}')

# Gate the pipeline on all three finishing successfully.
sift-cli wait job "$JOB_A" "$JOB_B" "$JOB_C"
```

The `import` command prints a `Job ID: <uuid>` line on the no-wait path
specifically so scripts can capture it.

## Inspecting jobs

- `sift-cli get jobs` lists the 50 most recent jobs, newest first. Add
  `--job-type` (`data-import`, `data-export`, `rule-evaluation`) or `--status`
  (`created`, `running`, `finished`, `failed`, `cancelled`, `cancel-requested`)
  to narrow it. `--limit` overrides the page size.
- `sift-cli get job <ID>` prints the full details on one job: type, status,
  timestamps, and failure details when the job failed.
- `sift-cli status job <ID>` is the scripting form. It prints one status word
  to stdout and exits with a code that reflects the job state.

### Exit codes for `status job`

| Code | Meaning                                    |
| ---- | ------------------------------------------ |
| `0`  | Job finished successfully.                 |
| `1`  | Job failed.                                |
| `2`  | Job was cancelled or cancel is requested.  |
| `3`  | Job is still running or has not started.   |

`wait job` uses `0` if every job finished and `1` if any job failed or was
cancelled; per-job status is printed to stderr for the non-success cases.
