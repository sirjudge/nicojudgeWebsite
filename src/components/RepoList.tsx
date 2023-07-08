import {GetData} from "../functions/GetData";
import React, { useState } from 'react'
import {ClipLoader} from "react-spinners";



async function GetRepoData(url){
    let returnRepoData: any[] = [];
    await GetData(url)
        .then(data => {
            for(var i in data)
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

    if (repoData !== undefined){
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
        return (
            <div id="repoDataTable">
                <p> data totally returned</p>
                <p>dataLength:{JSON.stringify(repoData)}</p>
            </div>
        );
    }
}