const fs = require('fs');

const values =
    fs.readFileSync(process.argv[2], 'ascii').split('\n').map(v => parseInt(v));

ricerca: {
  for (let i = 0; i < values.length; i++) {
    for (let j = i + 1; j < values.length; j++) {
      if (values[i] + values[j] == 2020) {
        console.log(`1: ${values[i] * values[j]}`);
        break ricerca;
      }
    }
  }
}

ricerca2: {
  for (let i = 0; i < values.length; i++) {
    for (let j = i + 1; j < values.length; j++) {
      for (let k = j + 1; k < values.length; k++) {
        if (values[i] + values[j] + values[k] == 2020) {
          console.log(`2: ${values[i] * values[j] * values[k]}`);
          break ricerca2;
        }
      }
    }
  }
}