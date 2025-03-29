const express = require('express');
const dbController = require('./db.controller');

// *****************************************************
// 
// This functions connect the app to Postgres database
// 
// *****************************************************
const insertUser = async (req, res) => {
    const { name, email } = req.body;
    try {   
        await dbController.insertUser(name, email);
        res.status(201).json({ message: 'User created successfully' });
    } catch (error) {
        res.status(500).json({ message: 'Error creating user', error });
    }
};

const getUsers = async (req, res) => {
    await dbController.getUsers()
        .then((users) => {
            res.status(200).json(users);
        })
        .catch((error) => {
            res.status(500).json({ message: 'Error retrieving users', error });
        });
};


// *****************************************************
//  Exports Functions
// *****************************************************
exports.insertUser = insertUser;
exports.getUsers = getUsers;