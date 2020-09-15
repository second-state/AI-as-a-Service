const { infer } = require('../pkg/mobilenet_service_lib.js');
const fs = require('fs');

var model_data = fs.readFileSync("mobilenet_v2_1.4_224_frozen.pb");
var label_data = fs.readFileSync("imagenet_slim_labels.txt");
var img_src = fs.readFileSync("grace_hopper.jpg");
console.log("Done reading file");
console.log("Result is: ", infer(model_data, label_data, img_src));
console.log("Done inference");
