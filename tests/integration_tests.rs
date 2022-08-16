#[test]
fn text_read_and_build(){
    let input = shiptracker::models::input::Input::new(&[
                                                       "target/debug/shiptracker".to_string(),
                                                       "tests/data/test_data.txt".to_string()]).unwrap();

    let reader = shiptracker::reader::reader(input).unwrap();
    let built_data = shiptracker::builder::data_builder(reader).unwrap();
    shiptracker::writer::generate_report(&built_data);
}

#[test]
fn csv_read_and_build(){
    let input = shiptracker::models::input::Input::new(&[
                                                       "target/debug/shiptracker".to_string(),
                                                       "tests/data/test_data.csv".to_string()]).unwrap();

    let reader = shiptracker::reader::reader(input).unwrap();
    let built_data = shiptracker::builder::data_builder(reader).unwrap();
    shiptracker::writer::generate_report(&built_data);
}
