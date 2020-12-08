const fs = require('fs');

function* readLines(filename, separator = '\n') {
  const file = fs.readFileSync(filename, {encoding : 'ascii'});
  let start = 0;

  let i = 0;
  while (start < file.length) {
    i = file.indexOf(separator, start)
    if (i == -1) {
      i = file.length;
    }
    yield file.substr(start, i - start);
    start = i + separator.length;
  }
}

module.exports = {readLines};
