
# My CLI Tool

A command-line tool built in Rust to perform various tasks efficiently.

## Installation

To install and use this tool, follow the steps below:

### Prerequisites
- Rust installed on your system. You can install Rust by following the instructions [here](https://www.rust-lang.org/tools/install).

### Build from Source
1. Clone the repository:
   ```bash
   git clone https://github.com/nogibjj/zichun-miniproject-7.git
   cd zichun-miniproject-7
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. The compiled binary will be located in the `target/release/` directory. You can copy it to a location in your `PATH` for global use:
   ```bash
   cp target/release/my_cli_tool /usr/local/bin/
   ```

### Download Precompiled Binary
Alternatively, you can download the precompiled binary from the latest GitHub release:
1. Go to the [Releases](https://github.com/nogibjj/zichun-miniproject-7/releases) page.
2. Download the binary for your operating system.
3. Move the binary to a directory in your `PATH`:
   ```bash
   mv my_cli_tool /usr/local/bin/
   chmod +x /usr/local/bin/my_cli_tool
   ```

## Usage

Once installed, you can use the tool from the terminal:

```bash
my_cli_tool --input <file>
```

### Example

```bash
my_cli_tool --input data.csv
```

This will process the input file and output the results.

## Features
- Accepts input files as arguments.
- Flexible argument parsing using the `clap` crate.
- Simple and efficient Rust implementation.

## Contribution

Feel free to open issues or pull requests to contribute to the project!

## License

This project is licensed under the MIT License.
