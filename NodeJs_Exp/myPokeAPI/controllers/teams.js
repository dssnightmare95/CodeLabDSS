let teamsDatabase = {};

const bootstrapTeam = (userId) => {
    teamsDatabase[userId] = [];
}

const getTeamOfUser = (userId) => {
    return teamsDatabase[userId];
}

const addPokemon = (userId, pokemonName, pokedexNumber) => {
    teamsDatabase[userId].push({name: pokemonName, pokedexNumber: pokedexNumber});
}

const setTeam = (userId, team) => {
    teamsDatabase[userId] = team;
}

const cleanUpTeams = () => {
    for(let user in teamsDatabase) {
        teamsDatabase[user] = [];
    }
}

const removePokemon = (userId, pokeId) => {
    let team = teamsDatabase[userId];
    team.splice(pokeId, 1);
}

exports.bootstrapTeam = bootstrapTeam;
exports.addPokemon = addPokemon;
exports.setTeam = setTeam;
exports.getTeamOfUser = getTeamOfUser;
exports.cleanUpTeams = cleanUpTeams;
exports.removePokemon = removePokemon;