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

- Keeping with Rust's convention, unit tests are still stored at the end of their associated files, while integration tests
or E2E tests now reside in the /test directory. I forgot to elaborate last time but so long as the Rust compiler sees
the '#[cfg(test)]' procedural macro above a tests module, it will know not to execute at compile time.

- This time 'lib.rs' simply allows the tests crate to access the internal modules of the application, the Rust compiler recognizes
this file as a special utility.

- I'm still a little shakey with writing proper unit tests, I'm reusing blocks of code to run them and I'm not sure
exactly how to improve this right now.

- Wasn't sure whether or not to write tests for my 'formatter' and 'writer' files as they just handle string formatting and
printing. Both were included in the integration tests.
