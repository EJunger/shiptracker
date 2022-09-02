# Shiptracker
by: Emma Junger

    shiptracker <FILE> <OPTIONAL:FILE>

Parses .txt & .csv records and generates a report detailing the total shipment times, layover times, and longest step taken.

Build Instructions:

    $ cargo build

    $ cargo run <FILE> <OPTIONAL:FILE> || $ ./target/debug/shiptracker <FILE> <OPTIONAL:FILE>


Project Structure Summary:

    - reader.rs: Parse raw file data into string data
    - builder.rs: Builds complete data records from string data
    - calc.rs: Perform various arithmetic operations on record data
    - formatter.rs: Format various calculations
    - writer.rs: Pretty print a report

    - models{Input, Record, Transfer}: Associated data entities.

Notes:
- Now handles '.csv' files!
