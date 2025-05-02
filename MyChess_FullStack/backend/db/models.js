const {DataTypes, UUID, UUIDV4} = require('sequelize');
const sequelize = require('./database').sequelize;

const DockerLabDB = sequelize.define('users', {
    id: {
        type: UUID,
        defaultValue: DataTypes.UUIDV4,
        primaryKey: true
    },
    name: {
        type: DataTypes.STRING,
        allowNull: false
    },
    email: {
        type: DataTypes.STRING,
        allowNull: false,
        unique: true
    }
}, {
    timestamps: false,
    tableName: 'users'
});

exports.DockerLabDB = DockerLabDB;