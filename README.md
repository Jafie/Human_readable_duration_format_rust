# Human Readable Duration Format

This tool generates a human-readable string representing a duration in years, months, days, hours, minutes, and seconds from a simple string input.

## Features

- Converts a duration string into a human-readable format.
- Supports years, months, days, hours, minutes, and seconds.
- Easy to use and integrate into your projects.

## Installation

To use this tool, you need to have Rust installed. You can install Rust from [here](https://www.rust-lang.org/tools/install).

Clone the repository:

```sh
git clone https://github.com/Jafie/human_readable_duration_format_rust.git
cd human_readable_duration_format
```

Build the project:

```sh
cargo build --release
```

## Usage

To use the tool, run the following command:

```sh
cargo run -- your_duration_string
```

Replace `your_duration_string` with the duration string you want to convert. For example:

```sh
cargo run -- "4895456"
```

This will output:

```
56 days, 15 hours, 50 minutes and 56 seconds
```

## Examples

Here are some example inputs and their corresponding outputs:

- Input: `159181551865156`
    - Output: `5047613 years, 327 days, 12 hours, 19 minutes and 16 seconds`
- Input: `4515915`
    - Output: `52 days, 6 hours, 25 minutes and 15 seconds`
- Input: `0`
    - Output: `now`

## License

This project is licensed under the MIT License.

## Contact

For any questions or suggestions, please open an issue on GitHub.
