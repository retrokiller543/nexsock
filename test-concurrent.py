#!/usr/bin/env python3
import asyncio
import time
import subprocess
import argparse
from datetime import datetime

async def run_command(session_id):
    """Run the nexsock status command and measure its execution"""
    start_time = time.time()

    try:
        process = await asyncio.create_subprocess_shell(
            "nexsock status ferric_api",
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE
        )

        stdout, stderr = await process.communicate()
        end_time = time.time()
        duration = end_time - start_time

        return {
            'session_id': session_id,
            'success': process.returncode == 0,
            'duration': duration,
            'output': stdout.decode() if stdout else None,
            'error': stderr.decode() if stderr else None
        }
    except Exception as e:
        end_time = time.time()
        return {
            'session_id': session_id,
            'success': False,
            'duration': end_time - start_time,
            'output': None,
            'error': str(e)
        }

async def run_concurrent_tests(num_concurrent, num_requests):
    """Run multiple concurrent tests"""
    print(f"\nStarting test with {num_concurrent} concurrent connections, {num_requests} total requests")
    print(f"Time: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
    print("-" * 50)

    results = []
    total_start = time.time()

    for batch in range(0, num_requests, num_concurrent):
        batch_size = min(num_concurrent, num_requests - batch)
        tasks = [run_command(i + batch) for i in range(batch_size)]
        batch_results = await asyncio.gather(*tasks)
        results.extend(batch_results)

        # Progress update
        print(f"Completed {min(batch + num_concurrent, num_requests)}/{num_requests} requests")

    total_duration = time.time() - total_start

    # Analyze results
    successful = [r for r in results if r['success']]
    failed = [r for r in results if not r['success']]

    durations = [r['duration'] for r in results]
    avg_duration = sum(durations) / len(durations)
    max_duration = max(durations)
    min_duration = min(durations)

    print("\nTest Results:")
    print(f"Total time: {total_duration:.2f} seconds")
    print(f"Successful requests: {len(successful)}")
    print(f"Failed requests: {len(failed)}")
    print(f"Average request duration: {avg_duration:.3f} seconds")
    print(f"Min request duration: {min_duration:.3f} seconds")
    print(f"Max request duration: {max_duration:.3f} seconds")
    print(f"Requests per second: {num_requests/total_duration:.2f}")

    if failed:
        print("\nError samples:")
        for error in failed[:3]:  # Show first 3 errors
            print(f"Session {error['session_id']}: {error['error']}")

    return results

def main():
    parser = argparse.ArgumentParser(description='Test nexsock daemon with concurrent connections')
    parser.add_argument('--concurrent', type=int, default=10,
                       help='Number of concurrent connections')
    parser.add_argument('--total', type=int, default=100,
                       help='Total number of requests to make')

    args = parser.parse_args()

    asyncio.run(run_concurrent_tests(args.concurrent, args.total))

if __name__ == "__main__":
    main()