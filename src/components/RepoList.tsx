import {GetData} from "../functions/GetData";
import React, { useState } from 'react'
import {ClipLoader} from "react-spinners";



async function GetRepoData(url:string){
    let returnRepoData: any[] = [];
    await GetData(url)
        .then(data => {
            for(let i in data)
                returnRepoData.push([i, data[i]]);
            return returnRepoData;
        });
}

export default function RepoList() {
    const [repoData,setRepoData] = useState();
    const [dataRequested,setDataRequested] = useState(false);

    const url = "https://api.github.com/users/sirjudge/repos";
    let [color, setColor] = useState("#ffffff");

    if (!dataRequested){
        setDataRequested(true);
        GetRepoData(url).then(response =>
        {
            setRepoData(response);
        });
    }

    if (!dataRequested || repoData !== undefined){
        return(
            <div>
                <ClipLoader
                    color={color}
                    size={150}
                    aria-label="Loading Spinner"
                    data-testid="loader"
                />
            </div>
        );
    }
    else {
       console.log(repoData);
       if (repoData === undefined)
        return (
           <div>
                <p> data totally returned</p>
                <p>dataLength:{JSON.stringify(repoData)}</p>
            </div>
        );
       else {
           return (
               <div id="repoDataTable">
                  <table>
                      <tbody>
                          <tr>this is a data Row</tr>
                      </tbody>
                  </table>
               </div>
           );
       }
    }
}