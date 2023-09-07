import React from 'react';
import RepoList from "./RepoList";
import DiceRoller from "./DiceRoller";
import { useState } from 'react';

export default class ApplicationSwitcher extends React.Component {




    render(){
        
        const [selectedAppValue, setCurrentValue] = useState("RepoList");
        return(
            <div className="ApplicationSwitcher">
                <select id="appSelector" value={selectedAppValue} defaultValue={"RepoList"}> 
                    <option value="RepoList">RepoList</option>
                    <option value="DiceRoller">DiceRoller</option>
                </select>
                <br></br>
                <button onClick = {() => displaySelectedComponent(selectedAppValue)}>Display</button>
                
                <div id="DiceRollerComponent" hidden={true}> 
                <RepoList></RepoList> 
                </div>

                <div id="RepoListComponent" hidden = {true}> 
                <DiceRoller></DiceRoller>
                </div>
            </div>
          );
    }
}


function displaySelectedComponent(selectedAppName : string){
    if(selectedAppName === "RepoList"){
        document.getElementById("DiceRollerComponent").hidden = true;
        document.getElementById("RepoListComponent").hidden = false;
    }
    else if(selectedAppName === "DiceRoller"){
        document.getElementById("DiceRollerComponent").hidden = false;
        document.getElementById("RepoListComponent").hidden = true;
    }
    
}
