const express = require('express');
const fileUpload = require('express-fileupload');
const { infer } = require('../pkg/mobilenet_std_lib.js');

const app = express();
const host = '0.0.0.0';
const port = 8080;
app.use(express.static('public'));
app.use(fileUpload());
// app.use(express.urlencoded({ extended: false }));

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
  console.time(image_file.name);
  var result = JSON.parse( infer(image_file.data) );
  console.timeEnd(image_file.name);

  res.send(
    'Detected <b>' +
      result[0] +
      '</b> with <u>' +
      result[1] +
      '</u> confidence.'
  );
});

app.listen(port, host, () =>
  console.log(`Listening at http://${host}:${port}`)
);
