const Sequelize = require('sequelize');
const { Pool } = require('pg');

const sequelize = new Sequelize(process.env.DATABASE_URL, {
    dialect: 'postgres',
    logging: false
});

sequelize.authenticate()
    .then(() => console.log('📡 Connection has been established successfully.'))
    .catch(err => console.error('❌ Unable to connect to the database:', err));

sequelize.sync()
    .then(() => console.log('📡 Database & tables created!'))
    .catch(err => console.error('❌ Error creating database & tables:', err));
    
exports.sequelize = sequelize;