use chrono::NaiveDate;
use csv::WriterBuilder;
use serde::Serialize;
use std::io::Write;
use std::path::Path;
use std::fs::OpenOptions;

#[derive(Serialize)]
struct Row {
    label: String,
    values: Vec<String>,
}

fn log_vec(v:Vec<String>, l:&str, p:&str) -> Result<(), std::io::Error> {

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

fn f_vec_f64(v:Vec<f64>) -> Vec<String> {
    v.iter().map(|&x| format!("{:?}",x)).collect::<Vec<_>>()
}

fn f_vec_date(v:Vec<NaiveDate>) -> Vec<String> {
    v.iter().map(|&x| format!("{}",x)).collect::<Vec<_>>()
}


fn main() -> Result<(), std::io::Error> {

    let conv_adj = 0.006;
    let p = format!( r"c:\tmp\my_csv_{:04}.csv", conv_adj );

    let u = vec![0.12, 0.65, 0.98];
    log_vec(f_vec_f64(u), "f32", &p)?;

    let d : Vec<NaiveDate> = vec![NaiveDate::from_ymd(2015, 3, 14), NaiveDate::from_ymd(2016, 4, 14)];
    log_vec(f_vec_date(d), "date", &p)?;

    Ok(())
}


