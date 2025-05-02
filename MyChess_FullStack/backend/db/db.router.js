const express = require('express');
const router = express.Router();

const dbHttp = require('./db.http');

router.route('/newUsers')
    .post(dbHttp.insertUser)

router.route('/getUsers')
    .get(dbHttp.getUsers)

exports.router = router;