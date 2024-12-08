console.log("Hello world!");

var string_tst = "Hola mundo!";
string_tst += " hahaha "
console.log(string_tst, "en espanol");

var inic = 0;
var final = 100;
var comp = 56;

var estaEnElRango = inic <= comp && comp <= final;
if (estaEnElRango) {
    console.log("Cierto!")
}

var myFunction = function() {
    console.log("Im function!");
}

myFunction();