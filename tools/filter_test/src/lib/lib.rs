//Needed to serialize data for Python
extern crate serde;
extern crate serde_pickle;

//Needed to write stuff to files, and to delete temporary files
use std::env;
use std::fs::File;
use std::io::prelude::*;

//Needed to call a Python script from Rust
use std::process::Command;

//Needed for unit tests
extern crate rand;

pub fn plot(basic_vectors: &(Vec<f64>, Vec<f64>)) {
    //Serialize the data and write it to a pickle file.
    let global_path = env::current_dir().unwrap();
    let ser_path: &str = &format!("{}/output.tmp.pkl", global_path.to_str().unwrap());
    let serialized = serde_pickle::to_vec(basic_vectors, true).unwrap();
    let mut buffer = File::create(ser_path).unwrap();
    match buffer.write(&serialized) {
        Err(_x) => panic!(
            "Failed to write serialized data to file at path = \"{}\"",
            ser_path
        ),
        _ => {}
    };

    //Now we send python the path to the serialized file and let matplotlib create a graph of it for us.
    //let py_path = "src/plotter.py";

    let py_path = &format!("{}/plotter.py", global_path.to_str().unwrap());

    let _new_command = Command::new("python3")
        .arg(py_path)
        .arg(ser_path)
        .status()
        .expect("Failed to start plotter.");

    //Now we want to delete the file, we don't want tons of loose ends around.
    match std::fs::remove_file(ser_path) {
        Err(_x) => panic!("Failed to remove temporary file at path = \"{}\"", ser_path),
        _ => {}
    };
}

#[cfg(test)]
mod tests {
    use rand::{thread_rng, Rng};
    #[test]
    fn test_serialize() {
        //Can we serialize and deserialize something?
        let mut a: Vec<f64> = vec![];
        let mut b: Vec<f64> = vec![];

        let mut rng = thread_rng();

        for i in 0..100000 {
            let x: f64 = rng.gen();
            let y: f64 = rng.gen();

            a.push(x);
            b.push(y);
        }

        let serialized = serde_pickle::to_vec(&(&a, &b), true).unwrap();
        let deserialized = serde_pickle::from_slice(&serialized).unwrap();
        assert_eq!((a, b), deserialized);
    }

    #[test]
    fn test_fs() {}
}
