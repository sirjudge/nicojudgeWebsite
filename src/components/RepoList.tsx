import React, { useState } from 'react'
import {ClipLoader} from "react-spinners";

async function GetRepoData(url:string){
    // let returnRepoData: any[] = [];
    const fetchPromise = await fetch(url, {
        method: 'GET'
    });

   const repoJson = await fetchPromise.json();
   return repoJson;
}

export default function RepoList() {
    const [repoData,setRepoData] = useState();
    const [dataRequested,setDataRequested] = useState(false);
    const [dataReturned,setDataReturned] = useState(false);
    const url = "https://api.github.com/users/sirjudge/repos";
    let [color, setColor] = useState("#ffffff");

    if (!dataRequested){
        setDataRequested(true);
        GetRepoData(url).then(response =>
        {
            setRepoData(response);
            setDataReturned(true);
        });
    }

    if (!dataReturned){
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
        console.log("should be data here");
        console.log(repoData);
        return (
           <div id="repoDataTable">
               <table>
                  <tbody>
                  {
                      repoData.map(function (repo:any){
                          return(
                            <tr>
                                <td>
                                    <a href={repo.html_url}> {repo.name}</a>
                                </td>
                                <td>
                                    {repo.description}
                                </td>
                            </tr>
                          );
                      })
                  }
                  </tbody>
              </table>
           </div>
       );
    }
}