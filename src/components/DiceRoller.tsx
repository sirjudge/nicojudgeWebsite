
export default function DiceRoller() {
    return (
        <div>
            <h1>Dice Roller</h1>
            <button onClick= {() =>  rollDice(3)}>Roll D3</button>
            <button onClick= {() => rollDice(4)}>Roll D4</button>
            <button onClick= {() => rollDice(8)}>Roll D8</button>
            <button onClick= {() => rollDice(12)}>Roll D12</button>
            <button onClick= {() => rollDice(20)}>Roll D20</button>
            <button onClick= {() => rollDice(100)}>Roll D100</button>
            <p> Rolls: <span id="rollOutput"></span></p>
        </div>
    )
}

function rollDice(diceType: number) {
    let roll = Math.floor(Math.random() * diceType) + 1;
    return roll;
}
