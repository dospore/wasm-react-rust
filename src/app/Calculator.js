import React, { useState } from 'react';
import './Calculator.css';

export const Calculator = ({ nativeModule }) => {
  const [calculateString, setCalculateString] = useState("")
  const [result, setResult] = useState(undefined)

  const calculate = () => {
    let r = nativeModule.calculate(calculateString)
    setResult(r);
  }

  return (
    <div className="Calculator">
      <div className="controls">
        <input value={calculateString} onChange={(e) => setCalculateString(e.target.value) }/>
        <button onClick={calculate}>Calculate</button>
      </div>
      <div className="result">
        {result && <div>Result: {result}</div>}
      </div>
    </div>
  )

}

export default Calculator;
