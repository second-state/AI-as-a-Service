use serde_json::json;
use std::env;
use std::error::Error;
use std::io::{self, Read};
use tensorflow::{Graph, ImportGraphDefOptions, Session, SessionOptions, SessionRunArgs, Tensor};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let model_size: u64 = args[1].parse::<u64>().unwrap();
    let model_input: &str = &args[2];
    let model_output: &str = &args[3];
    let model_params: &str = &args[4];
    let img_width: u64 = args[5].parse::<u64>().unwrap();
    let img_height: u64 = args[6].parse::<u64>().unwrap();

    let mut buffer: Vec<u8> = Vec::new();
    let mut model: Vec<u8> = Vec::new();
    let mut flattened: Vec<f32> = Vec::new();

    // Parse stdin: vactor<flattended u8>.
    let mut i : u64 = 0;
    io::stdin().read_to_end(&mut buffer)?;
    for num in buffer {
        if i < model_size {
            model.push(num);
        } else {
            flattened.push(num.into());
        }
        i = i + 1;
    }

    // Load up the graph as a byte array and create a tensorflow graph.
    let mut graph = Graph::new();
    graph.import_graph_def(&*model, &ImportGraphDefOptions::new())?;

    // The `input` tensor expects BGR pixel data.
    let input = Tensor::new(&[img_height, img_width, 3]).with_values(&flattened)?;
    // Set the params
    let mut params: Vec<(&str, Tensor<f32>)> = Vec::new();
    let ps: serde_json::Value = serde_json::from_str(model_params).unwrap();
    for (key, value) in ps.as_object().unwrap() {
        let mut vec = Vec::new();
        for v in value.as_array().unwrap().iter() {
            vec.push(v.as_f64().unwrap() as f32);
        }
        if vec.len() == 1 {
            params.push((key, Tensor::new(&[]).with_values(&vec)?));
        } else {
            params.push((key, Tensor::new(&[vec.len() as u64]).with_values(&vec)?));
        }
    }

    let mut args = SessionRunArgs::new();
    args.add_feed(&graph.operation_by_name_required(model_input)?, 0, &input);
    for (key, value) in &params {
        args.add_feed(&graph.operation_by_name_required(key)?, 0, &value);
    }

    // Output tensor
    let output = args.request_fetch(&graph.operation_by_name_required(model_output)?, 0);

    // Execute the model
    let session = Session::new(&SessionOptions::new(), &graph)?;
    session.run(&mut args)?;

    // Return Tensor value
    let res: Tensor<f32> = args.fetch(output)?;

    let mut i = 0;
    let mut json_vec: Vec<f32> = Vec::new();
    while i < res.len() {
        json_vec.push(res[i]);
        i += 1;
    }
    let json_obj = json!(json_vec);
    println!("{}", json_obj.to_string());
    Ok(())
}
