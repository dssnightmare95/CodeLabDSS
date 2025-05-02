const express = require('express');


const app = express();

const dbRouter = require('./db/db.router').router;

app.use(express.json());

app.get('/', (req, res) => {
  res.send('Hello World!**');
});

app.use('/db', dbRouter);

app.listen(3000, () => {
  console.log('Server is running on port 3000 🚀');
});
