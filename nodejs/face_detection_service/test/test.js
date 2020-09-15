const { infer } = require('../pkg/face_detection_service_lib.js');
const fs = require('fs');

var img_src = fs.readFileSync("solvay.jpg");
console.log("Done reading file");
var img_res = infer("20", img_src);
console.log("Done inference");
fs.writeFileSync("res.png", img_res);
console.log("Done writing file");
