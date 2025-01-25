# Objetivo:
Definir una API para gestionar nuestro equipo pokemon

# Acciones:
- Identificarnos
- Anadir pokemon a nuestro equipo
- Eliminar pokemon de nuestro equipo
- Consultar informacion de nuestro equipo
- Intercambiar el orden de nuestros pokemon

# REST Design:
- Anadir pokemon: POST /team/pokemons
- Consultar equipo: GET /team
- Eliminar pokemon: DELETE /team/pokemons/:id
- Intercambiar el orden de nuestro pokemon: PUT /team
- Sistema de credenciales