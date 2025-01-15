import {KukuVisualFlowLoggerManual} from "./KukuVisualFlowLogger/index.js";
const logger = new KukuVisualFlowLoggerManual("http://127.0.0.1:8080/");
function calculateInterest(amount) {
    logger.START("calculateInterest")
    logger.log("calculateInterest", `Calculating interest for ${amount}`)
    const interest = amount * 0.1;
    logger.log("calculateInterest" `Interest = ${interest}`);
    return interest;
    logger.END("calculateInterest");
}

function addMoney(balance, deposit) {
    logger.START("addMoney")
    logger.log("addMoney",`Adding money (${deposit}) to balance ${balance}`)
    const updatedBalance = balance + deposit;
    logger.log("addMoney",`Balance = ${updatedBalance}`)
    return updatedBalance;
}

function getCompoundInterest(initialAmount, years) {
    logger.log(`Getting CI for ${initialAmount}, ${years}`)
    if (years === 0) {
        logger.log(`Year == 0. Returning ${initialAmount}`);
        return initialAmount;
    }

    const interest = calculateInterest(initialAmount);
    logger.log(`interest = ${interest}`)
    const newBalance = addMoney(initialAmount, interest);
    logger.log(`Updated Balance = ${newBalance}`)
    const ci = getCompoundInterest(newBalance, years - 1);
    logger.log(`CI = ${ci}`);
    return ci;
}

//Logs transaction to database. Returns true if successful
function logTransaction(message) {
    const timestamp = new Date().toISOString();
    const msg = `${timestamp}: ${message}`;  // Return value not used by caller
    logger.log(`Logging transaction ${msg}`)
    return true;
}

function calculateInvestment(balance) {
    logger.log(`Calculating investment for ${balance}`)
    const finalAmount = getCompoundInterest(balance, 2);
    logger.log(`final Amount After investing = ${finalAmount}`);
    logTransaction(`Investment matured: $${finalAmount}`);
}

calculateInvestment(1000);


