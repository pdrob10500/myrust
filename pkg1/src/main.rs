use std::error::Error;
use csv::WriterBuilder;
use serde::Serialize;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::ffi::OsStr;

#[derive(Serialize)]
struct Row {
    label: String,
    values: Vec<f64>,
}

fn log_vec(v:Vec<f64>, p:&str) -> Result<(), std::io::Error> {

    let fname = Path::new(p);

    let mut wtr = WriterBuilder::new()
        .has_headers(false)
        .from_writer(vec![]);
    wtr.serialize(Row {
        label: "foo".to_string(),
        values: v,
    })?;

    let data = String::from_utf8(wtr.into_inner().expect("csv writer error")).expect("csv writer to string error");

    let mut writer = csv::Writer::from_path(fname).unwrap();
    writer.write_record(&[data.as_bytes()]);

    //let mut wrt = Writer::from_path("C:\\tmp\\foo.csv")?;
    //wrt.write_record(&v)?;
    //wrt.flush()?;

    Ok(())
}


fn main() -> Result<(), std::io::Error> {

    let v = vec![0.12, 0.65, 0.98];
    let p = r"c:\tmp\my_csv.csv";
    log_vec(v, p)?;

    Ok(())
}


