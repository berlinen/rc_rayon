const USAGE: &str = "
Usage: life bench [--size N] [--gens N] [--skip-bridge]
       life play [--size N] [--gens N] [--fps N] [--skip-bridge]
       life --help
Conway's Game of Life.

Commands:
    bench           Run the benchmark in different modes and print the timings.
    play            Run with a max frame rate and monitor CPU resources.
Options:
    --size N        Size of the game board (N x N) [default: 200]
    --gens N        Simulate N generations [default: 100]
    --fps N         Maximum frame rate [default: 60]
    --skip-bridge   Skips the tests with par-bridge, as it is much slower.
    -h, --help      Show this message.
";
