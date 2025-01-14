const {KukuVisFlowLogger} = require("./KukuVisualFlowLogger")
const logger = new KukuVisFlowLogger("");

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
    const num = 4;
    logger.LOG("Calculate", `num = ${num}`);
    logger.STORE("Calculate")
    const square1 = square(num);
    logger.LOG("Calculate", `Square = ${square}`);
    double(square1);
    logger.LOG("Calculate", "Calculation Complete");
    logger.END("Calculate");
    return "Complete";

}
logger.run(calculate);
logger.run(calculate);
