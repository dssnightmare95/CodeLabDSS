const sequelize = require('./database');
const { QueryTypes } = require('sequelize');

const DockerLabDB = require('./models').DockerLabDB;

const insertUser = async (name, email) => {
    try {
        const newUserSave = await DockerLabDB.create({name: name, email: email});
        console.log('✅ Usuario insertado correctamente', newUserSave.toJSON());
    } catch (error) {
        console.error('❌ Error al insertar usuario:', error);
        throw error;
    }
};

const getUsers = async () => {
    try {
        const users = await DockerLabDB.findAll({
            attributes: ['id', 'name', 'email'],
            raw: true,
        });
        console.log('✅ Usuarios obtenidos correctamente');
        return users;
    } catch (error) {
        console.error('❌ Error al obtener usuarios:', error);
        throw error;
    }
};


exports.insertUser = insertUser;
exports.getUsers = getUsers;