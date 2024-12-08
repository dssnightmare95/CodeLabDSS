const readline = require('readline');

const rl = readline.createInterface({ input: process.stdin, output: process.stdout });

function imprimirNombre(nombre) {
    console.log("Hola", nombre);
}

rl.question("Dame tu nombre ", (nombre) =>
    {
        imprimirNombre(nombre);
        rl.close();
    }
);