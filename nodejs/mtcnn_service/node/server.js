const express = require('express');
const fileUpload = require('express-fileupload');
const { infer } = require('../pkg/mtcnn_service_lib.js');
const fs = require('fs');
const { v4: uuidv4 } = require('uuid');

const app = express();
const host = '0.0.0.0';
const port = 8080;
app.use(express.static('public'));
app.use(fileUpload());

app.get('/', (req, res) => res.redirect('/index.html'));

app.post('/infer', function (req, res) {
  if (!req.files || Object.keys(req.files).length === 0) {
    return res.status(400).send('No files were uploaded.');
  }
  console.log(
    'Received ' +
      req.files.image_file.name +
      ' with size: ' +
      req.files.image_file.size
  );

  let image_file = req.files.image_file;
  var result_filename = uuidv4() + ".png";
  fs.writeFileSync("public/" + result_filename, infer(image_file.data));
  res.send('<img src="' +  result_filename + '"/>');

  // res.contentType('image/png');
  // res.send(result);
});

app.listen(port, host, () =>
  console.log(`Listening at http://${host}:${port}`)
);
