const chai = require('chai');
const chaiHttp = require('chai-http');

chai.use(chaiHttp);

const app = require('../app').app;

describe('Test e2e for mastermind course', () => {
    it('should return hello world', (done) => {
        chai.request(app)
            .get('/auth')
            .end((err, res) => {
                chai.assert.equal(res.text, 'Hello World!')
                done();
            });
    });
});