# Figure Skating Element Parser

## Project Description

This project is a parser for figure skating elements that complies with ISU (International Skating Union) standards. It automatically recognizes and classifies technical elements of figure skating, such as jumps, spins, and step sequences. The parser reads abbreviations used in the ISU judging system and converts them into a structured format suitable for analysis and use in various applications.

## Technical Description of the Parsing Process

The parser is based on grammar rules that adhere to ISU standards. It can recognize and classify the following categories of figure skating elements:
- **Jumps**: Single, double, triple, and quad jumps, each annotated with the jump type, for example:
1T — single toeloop,
3A — triple axel,
4Lz — quad lutz.
- **Spins**: Different spin positions, including standard spins, flying spins, and change-foot spins:
USp — upright spin,
LSp — layback spin,
FSSp — flying sit spin.
- **Step Sequences** та інші елементи: Choreographic and technical step sequences:
StSq — step sequence,
ChSq — choreographic step sequence.
- **Death Spirals and Twizzles:**:
Death spirals (e.g., FiDs, BoDs),
Twizzles (e.g., STw).

## Usage of Parsing Results

The parsing results can be used to create performance protocols for skaters, automate performance analysis, develop scoring systems, or serve as a basis for analytics and training programs.

## Requirements

- **Rust version** 1.56.0 or newer.
- **Libraries**: pest for parsing, thiserror for error handling, and anyhow for test convenience.

## Makefile Commands

The Makefile includes commands to simplify working with the project:

- **fmt**: formats code using cargo fmt.
- **clippy**: runs code linting with cargo clippy.
- **test**: runs unit tests using cargo test.
- **build**: builds the project.
- **run**: runs the project with elements.txt as an example.

## Installation and Running

1. Clone the repository:
```
git clone <repository URL>
cd figure_skating_element_parser
```

2. Build dependencies:
```
cargo build
```

3. Run the parser:
```
cargo run -- "elements.txt"
```

3. Execute all Makefile commands:
```
make all
```
