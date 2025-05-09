const express = require('express');
const middleware = require('./middlewares');

// Routes
const authRoutes = require('./auth/auth.router').router;
const teamsRoutes = require('./teams/teams.router').router;

const app = express();
middleware.setupMiddlewares(app);

const port = 3000;

app.get('/', (req, res) => {
    // req es la request, la peticion
    // res es la respuesta
    res.status(200).send('Hello World!')
});
app.use('/auth', authRoutes);
app.use('/teams', teamsRoutes);

app.listen(port, () => {
    console.log('Server started at port 3000');
})

exports.app = app;