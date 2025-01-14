import {KukuVisualFlowLoggerManual} from "./KukuVisualFlowLogger/index.js";

const logger = new KukuVisualFlowLoggerManual("http://127.0.0.1:8080/");

function square(num) {
    logger.START("Square");
    const result = num * num;
    logger.LOG("Square", `Result = ${result}`);
    logger.END("Square");
    return result;
}

function double(num) {
    logger.START("Double");
    const result = num * num;
    logger.LOG("Double", `Result = ${result}`);
    logger.END("Double");
    return result;
}

function calculate() {
    logger.START("Calculate");
    const num = Math.floor(Math.random() * 5);
    logger.LOG("Calculate", `num = ${num}`);
    const square1 = square(num);
    logger.STORE("Calculate", `Square1 = ${square1}`)
    double(square1);
    logger.LOG("Calculate", "Calculation Complete");
    logger.END("Calculate");
    return square1;

}

const result1 = await logger.run(calculate);
//const result2 = await logger.run(calculate);

console.log(`Result1 = ${result1} and result2 = ${"GG"}`);