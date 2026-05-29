#!/usr/bin/env python3
import sys
import os
import csv
import time
import subprocess

def main():
    if len(sys.argv) < 2:
        print("Usage: runner.py <solver_name>")
        sys.exit(1)

    solver_name = sys.argv[1]
    solver_path = os.path.join('solvers', solver_name, 'solve')

    if not os.path.exists(solver_path):
        print(f"Error: Solver executable not found at {solver_path}")
        sys.exit(1)

    index_path = os.path.join('inputs', 'index.csv')
    if not os.path.exists(index_path):
        print(f"Error: Index file not found at {index_path}")
        sys.exit(1)

    successes = 0
    failures = 0

    with open(index_path, 'r') as f:
        reader = csv.DictReader(f)
        for row in reader:
            test_file = row['test_file']
            expected_output = row['expected_output']

            test_file_path = os.path.join('inputs', test_file)
            if not os.path.exists(test_file_path):
                print(f"Warning: Test file {test_file_path} not found")
                failures += 1
                continue

            start_time = time.time()
            try:
                result = subprocess.run(
                    [solver_path, test_file_path],
                    capture_output=True,
                    text=True,
                    timeout=60 # Timeout just in case
                )
                end_time = time.time()
                elapsed = end_time - start_time

                if result.returncode != 0:
                    print(f"FAIL: {test_file} - Solver exited with code {result.returncode}")
                    print(f"Stderr: {result.stderr.strip()}")
                    failures += 1
                    continue

                output = result.stdout.strip()
                if output == expected_output:
                    print(f"PASS: {test_file} in {elapsed:.4f}s")
                    successes += 1
                else:
                    print(f"FAIL: {test_file} - Expected {expected_output}, got {output} in {elapsed:.4f}s")
                    failures += 1

            except subprocess.TimeoutExpired:
                print(f"FAIL: {test_file} - Solver timed out")
                failures += 1
            except Exception as e:
                print(f"FAIL: {test_file} - Error: {e}")
                failures += 1

    print(f"\nResults: {successes} passed, {failures} failed")
    if failures > 0:
        sys.exit(1)

if __name__ == '__main__':
    main()
