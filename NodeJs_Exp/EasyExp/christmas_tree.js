length = 15;

for (let i = 1; i <= length; i++){
    let espacio = ' '.repeat(length - i);
    let estrellas = '*'.repeat(2 * i - 1);
    console.log(espacio + estrellas + espacio);
}