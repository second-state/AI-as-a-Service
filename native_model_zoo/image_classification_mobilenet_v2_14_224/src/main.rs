use serde_json::json;
use std::error::Error;
use std::io::{self, Read};
use tensorflow::{Graph, ImportGraphDefOptions, Session, SessionOptions, SessionRunArgs, Tensor};

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer: Vec<u8> = Vec::new();
    let mut flattened: Vec<f32> = Vec::new();

    // Parse stdin: vactor<flattended u8>.
    io::stdin().read_to_end(&mut buffer)?;
    for num in buffer {
        flattened.push(num as f32 / 255.);
    }

    // Load up the graph as a byte array and create a tensorflow graph.
    let model = include_bytes!("mobilenet_v2_1.4_224_frozen.pb");
    let mut graph = Graph::new();
    graph.import_graph_def(&*model, &ImportGraphDefOptions::new())?;

    // The `input` tensor expects BGR pixel data.
    let input = Tensor::new(&[1, 224, 224, 3]).with_values(&flattened)?;
    let mut args = SessionRunArgs::new();
    // Add the input image to the input placeholder
    args.add_feed(&graph.operation_by_name_required("input")?, 0, &input);
    // Request the following outputs after the session runs.
    // let prediction = args.request_fetch(&graph.operation_by_name_required("MobilenetV2/Predictions/Reshape_1")?, 0);
    let prediction = args.request_fetch(&graph.operation_by_name_required("MobilenetV2/Predictions/Softmax")?, 0);

    let session = Session::new(&SessionOptions::new(), &graph)?;
    session.run(&mut args)?;

    let prediction_res: Tensor<f32> = args.fetch(prediction)?;

    // Print results.
    let mut i = 0;
    let mut json_vec: Vec<f32> = Vec::new();
    while i < prediction_res.len() {
        json_vec.push(prediction_res[i]);
        i += 1;
    }
    let json_obj = json!(json_vec);
    println!("{}", json_obj.to_string());
    Ok(())
}
