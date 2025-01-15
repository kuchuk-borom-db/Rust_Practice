import {KukuVisualFlowLoggerManual} from "./KukuVisualFlowLogger/index.js";

const logger = new KukuVisualFlowLoggerManual("http://127.0.0.1:8080/");

function calculateInterest(amount) {
    logger.START("calculateInterest");
    logger.log("calculateInterest", `Calculating interest for ${amount}`)
    const interest = amount * 0.1;
    logger.log("calculateInterest", `Interest = ${interest}`);
    logger.END("calculateInterest");
    return interest;
}

function addMoney(balance, deposit) {
    logger.START("addMoney");
    logger.log('addMoney', `Adding money (${deposit}) to balance ${balance}`)
    const updatedBalance = balance + deposit;
    logger.log('addMoney', `Balance = ${updatedBalance}`)
    logger.END("addMoney");
    return updatedBalance;
}

function getCompoundInterest(initialAmount, years) {
    logger.START("getCompoundInterest");
    logger.log('getCompoundInterest', `Getting CI for ${initialAmount}, ${years}`)
    if (years === 0) {
        logger.log('getCompoundInterest', `Year == 0. Returning ${initialAmount}`);
        logger.END("getCompoundInterest");
        return initialAmount;
    }

    const interest = calculateInterest(initialAmount);
    logger.STORE('getCompoundInterest', `interest = ${interest}`)
    const newBalance = addMoney(initialAmount, interest);
    logger.STORE('getCompoundInterest', `Updated Balance = ${newBalance}`)
    const ci = getCompoundInterest(newBalance, years - 1);
    logger.STORE('getCompoundInterest', `CI = ${ci}`);
    logger.END("getCompoundInterest");
    return ci;
}

//Logs transaction to database. Returns true if successful
function logTransaction(message) {
    logger.START("logTransaction");
    const timestamp = new Date().toISOString();
    const msg = `${timestamp}: ${message}`;  // Return value not used by caller
    logger.log('logTransaction', `Logging transaction ${msg}`)
    logger.END("logTransaction");
    return true;
}

function calculateInvestment(balance) {
    logger.START("calculateInvestment");
    logger.log('calculateInvestment', `Calculating investment for ${balance}`)
    const finalAmount = getCompoundInterest(balance, 2);
    logger.STORE('calculateInvestment', `final Amount = ${finalAmount}`);
    logTransaction(`Investment matured: $${finalAmount}`);
    logger.END("calculateInvestment");
}

await logger.run(() => {
    calculateInvestment(1000);
});


