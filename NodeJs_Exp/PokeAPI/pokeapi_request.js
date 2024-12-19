const axios = require('axios');

let pokemon = "ditto"
let route = "https://pokeapi.co/api/v2/pokemon/" + pokemon

function displayPokeInfo(jsonData){
    if (jsonData.name !== undefined){
        console.log("Pokemon name::", jsonData.name)
    }
    if (jsonData.abilities !== undefined){
        console.log(">> Abilities::");
        for (let abilities of jsonData.abilities){
            if (abilities.ability.name !== undefined){
                console.log("   -", abilities.ability.name)
            }
        }
    }
    if (jsonData.types !== undefined){
        console.log(">> Types::")
        for (let types of jsonData.types){
            if (types.type.name !== undefined){
                console.log("   -",types.type.name);
            }
        }
    }

}

function main (){
    axios.get(route)
        .then(response => {
            displayPokeInfo(response.data);
        })
        .catch(error => {
            console.error('Error en la solicitud:', error);
        })
}

main();