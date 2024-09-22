// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19; //solidity versions

contract SimpleStorage_old {

    //Basic types:: boolean, int, uint, address, bytes, string
    // All types have default values
    //bool exampleBool = false;
    uint256 numberUint = 28;
    //int256 numberint = -56;
    //string myName = "Diego";
    //address myAddress = 0x35B2103fdB8e459793D7885875960B810D810d84;
    //bytes32 favoritesBytes32 = "cat";
    uint256[] listOfFavoritesNumbers;

    struct Person {
        uint256 favoritNumber;
        string name;
    }

    Person public pat = Person({favoritNumber:7, name: "Pat"});

    //dynamic array
    Person[] public listOfPeople; 

    // Mapping
    mapping(string => uint256) public nameToFavoriteNumber;

    function store(uint256 _inputNumber) public {
        numberUint = _inputNumber;
    }

    // view :: read state from blockchain
    // pure :: 
    function retrieve() public view returns(uint256){
        return numberUint;
    }

    function addPerson( string memory _name, uint256 _favoriteNumber) public {
        Person memory newPerson = Person(_favoriteNumber, _name);
        listOfPeople.push(newPerson);

        nameToFavoriteNumber[_name] = _favoriteNumber;

    }

}

contract SimpleStorage {
    uint256 myFavoriteNumber;

    struct Person {
        uint256 favoriteNumber;
        string name;
    }
    // uint256[] public anArray;
    Person[] public listOfPeople;

    mapping(string => uint256) public nameToFavoriteNumber;

    function store(uint256 _favoriteNumber) public virtual {
        myFavoriteNumber = _favoriteNumber;
    }

    function retrieve() public view returns (uint256) {
        return myFavoriteNumber;
    }

    function addPerson(string memory _name, uint256 _favoriteNumber) public {
        listOfPeople.push(Person(_favoriteNumber, _name));
        nameToFavoriteNumber[_name] = _favoriteNumber;
    }
}

contract SimpleStorage2 {}

contract SimpleStorage3 {}

contract SimpleStorage4 {}