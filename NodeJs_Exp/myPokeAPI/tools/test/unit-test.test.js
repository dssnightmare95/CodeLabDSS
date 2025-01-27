const assert = require('chai').assert;

function sum(a,b){
    return a + b;
}


describe('Initial test for course', () => {
    it('should return 2', () => {
        let variable = sum(1,1);
        assert.equal(variable, 2);
    });
});