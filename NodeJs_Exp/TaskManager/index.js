const readline = require('readline');

// Primer modo: lectura de tareas 
let taskList = [];

function addTask(taskList, task){
    taskList.push(task);
}

function changeTaskStatus(taskList, taskIndex){
    if (taskIndex < taskList.length()){
        taskList[taskIndex].done = ~taskList[taskIndex].done;

    }
}


const rl = readline.createInterface({ input: process.stdin, output: process.stdout });

rl.question("Specify the ")

// Segundo modod: marcar las atreas realizadas