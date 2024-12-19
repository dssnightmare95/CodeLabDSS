import readline from 'readline';

const rl = readline.createInterface({ input: process.stdin, output: process.stdout });

// Define basic structure of Q&A
let QAList = [
    {
        "question": "Did you enjoy Mastermind courses?",
        "answerType": "str",
        "answers": ["Yes", "No"],
        "answer": ""
    },
    {
        "question": "What is your favorite color?",
        "answerType": "str",
        "answers": ["black", "white", "green", "red", "other"],
        "answer": ""
    },
    {
        "question": "How old are you?",
        "answerType": "int",
        "answers": [0, 100],
        "answer": ""
    },
    {
        "question": "Where are you from (country)?",
        "answerType": "str",
        "answers": ["Spain", "Costa Rica", "USA", "Other"],
        "answer": ""
    },
    {
        "question": "Are you married?",
        "answerType": "str",
        "answers": ["Yes", "No"],
        "answer": ""
    }
];

// Create functions to check valid answer
function validAnswer(answer, validAnswers, answerType){
    if (answerType == "int") {
        const num = Number(answer);
        if (!isNaN(num) && num >= validAnswers[0] && num <= validAnswers[1]){
            return true;
        }
        else {
            return false;
        }
    }
    else if (answerType == "str"){
        for (let ans of validAnswers){
            if (typeof answer === 'string' && answer.toUpperCase() == ans.toUpperCase()){
                return true;
            }
        }
        return false;
    }
}

// Create function to save answer
function saveAnswer(question, answer){
    question.answer = answer;
}

// Function to show question and accepted answers
function printQA(QA){
    console.log(QA.question);
    if (QA.answerType == "int"){
        console.log("Min.", QA.answers[0],"- Max.", QA.answers[1]);

    }
    else if (QA.answerType == "str"){
        for (let ansOpt of QA.answers){
            console.log("[ ]", ansOpt);
        }
    }
}

//Function to get answer
function getUserAns(){
    return new Promise((resolve) => {
         rl.question('Answer ', (respuesta) => {
             resolve(respuesta); 
             //rl.close(); 
         });
    });
}

for (let QAOpt of QAList) {
    let validAns = false;
    while (!validAns){
        printQA(QAOpt);
        let userAns = await getUserAns();
        if (validAnswer(userAns, QAOpt.answers, QAOpt.answerType)) {
            saveAnswer(QAOpt, userAns);
            validAns = true;
        }
        else {
            console.log("Answer not valid!");
        }
    }
}
rl.close(); 

console.log("*******  QA Summary! *******");
for (let QAOpt of QAList) {
    console.log("Question::", QAOpt.question);
    console.log("Answer::", QAOpt.answer);
}