import React from 'react';
import RepoList from "./RepoList";
import DiceRoller from "./DiceRoller";
import { useState } from 'react';

const [currentValue, setCurrentValue] = useState("RepoList");

export default function ApplicationSwitcher() {
   
   return(
        <div className="ApplicationSwitcher">
            <select id="appSelector" value={currentValue} defaultValue={currentValue}> 
                <option value="RepoList">RepoList</option>
                <option value="DiceRoller">DiceRoller</option>
            </select>
            <button onClick = {() => displaySelectedComponent()}>Display</button>
            <div id="DiceRollerComponent" hidden={true}> 
                <RepoList></RepoList> 
            </div>
            <div id="RepoListComponent" hidden = {true}> 
                <DiceRoller></DiceRoller>
           </div>
        </div>
   );
}

function displaySelectedComponent() {
    if (currentValue == "RepoList"){
        document.getElementById("RepoListComponent").hidden = false;
        document.getElementById("DiceRollerComponent").hidden = true;
    }
}
