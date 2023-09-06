import React, {useState} from 'react'
import {ClipLoader} from "react-spinners";
import '../Styles/repoData.css'
import '../Styles/GlobalStyle.css'

async function GetRepoData(url:string){
    const fetchPromise = await fetch(url, {
        method: 'GET'
    });
   const repoJson = await fetchPromise.json();

   GetRepoDataNew();
   return repoJson;
}

function GetRepoDataNew(){
    const url = "https://api.github.com/users/sirjudge/repos";
    fetch(url)
        .then(response => response.json())
        .then(repos => {
            // Sort repositories by creation date in descending order
            repos.sort((a, b) => new Date(b.created_at) - new Date(a.created_at));

            // Get the latest 5 repositories
            const latestRepos = repos.slice(0, 5);

            console.log('Latest 5 repositories:');
            latestRepos.forEach(repo => {
                console.log(`${repo.name} - ${repo.html_url}`);
            });
        })
        .catch(error => console.error('Error fetching repositories:', error));
}

export default function RepoList() {
    const [repoData,setRepoData] = useState();
    const [dataRequested,setDataRequested] = useState(false);
    const [dataReturned,setDataReturned] = useState(false);
    const url = "https://api.github.com/users/sirjudge/repos";
    let [color] = useState("#ffffff");

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
            <div className="centered">
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
        return (
           <div id="repoDataTable" className="centered">
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
