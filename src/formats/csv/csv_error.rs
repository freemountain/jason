use super::csv;
use super::serde_json;
use std::cmp::Ordering;
use std::str::Utf8Error;
//type Result<T> = std::result::Result<T, DoubleError>;

fn get_position(error: &CsvError) -> Option<(usize, String)> {
    match error {
        CsvError::UnkownFieldAt(row, field) => Some((*row, field.clone())),
        CsvError::JsonAt(_, row, field) => Some((*row, field.clone())),
        _ => None,
    }
}
fn cmp_errors(a: &CsvError, b: &CsvError) -> Ordering {
    let position_a = get_position(a);
    let position_b = get_position(a);

    match (position_a, position_b) {
        (None, None) => Ordering::Equal,
        (None, Some(_)) => Ordering::Less,
        (Some(_), None) => Ordering::Greater,
        (Some((row_a, field_a)), Some((row_b, field_b))) => {
            row_a.cmp(&row_b).then_with(|| field_a.cmp(&field_b))
        }
    }
}
fn format_errors(errors: &Vec<CsvError>) -> String {
    //let sorted = errors.clone();
    //sorted.sort_unstable_by(cmp_errors);

    errors
        .iter()
        .map(|e| format!("{}", e))
        .collect::<Vec<String>>()
        .join("\n")
}

quick_error! {
    #[derive(Debug)]
    pub enum CsvError {
        Utf8Error(err: Utf8Error) {
            from()
            display("Utf8Error error")
            cause(err)
        }
        Csv(err: csv::Error) {
            from()
            display("csv error")
            cause(err)
        }
        Json(err: serde_json::Error) {
            from()
            display("csv error")
            cause(err)
        }
        UnkownFieldAt(row: usize, field: String) {
            display("unkown fiel '{}' in row {}", field, row)
        }
        JsonAt(err: serde_json::Error, row: usize, field: String) {
            display("csv error at field '{}' in row {}: {}", field, row, err)
            cause(err)
        }
        Multiple(errors: Vec<CsvError>) {
            display (me) ->("multiple errors: {}", format_errors(errors))
        }
    }
}
