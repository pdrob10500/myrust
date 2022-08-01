use csv::WriterBuilder;
use serde::Serialize;
use std::io::Write;
use std::path::Path;
use std::fs::OpenOptions;

#[derive(Serialize)]
struct Row {
    label: String,
    values: Vec<f64>,
}

fn log_vec(v:Vec<f64>, l:&str, p:&str) -> Result<(), std::io::Error> {

    let fname = Path::new(p);

    let mut wtr = WriterBuilder::new()
        .has_headers(false)
        .from_writer(vec![]);
    wtr.serialize(Row {
        label: l.to_string(),
        values: v,
    })?;
    let data = String::from_utf8(wtr.into_inner().expect("csv writer error")).expect("csv writer to string error");

    let mut log = OpenOptions::new()
        .create(true)
        .append(true) // if file exists, add to the end
        .create_new(false)
        .open(fname)?;
    log.write_all(&data.as_bytes())?;

    Ok(())
}

fn main() -> Result<(), std::io::Error> {

    let conv_adj = 0.006;
    let v = vec![0.12, 0.65, 0.98];
    let p = format!( r"c:\tmp\my_csv_{:04}.csv", conv_adj );
    log_vec(v, "f32", &p)?;

    Ok(())
}


