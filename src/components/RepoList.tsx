import React, {useState} from 'react'
import {ClipLoader} from "react-spinners";
import '../Styles/repoData.css'
import '../Styles/GlobalStyle.css'

async function GetRepoData(url:string){
    var latestRepos; 
    url = "https://api.github.com/users/sirjudge/repos";
    await fetch(url)
        .then(response => response.json())
        .then(repos => {
                // Sort repositories by creation date in descending order
                //repos.sort((a, b) => new Date(b.created_at) - new Date(a.created_at));

                // Get the latest 5 repositories
                latestRepos = repos.slice(0, 5);
            })
    .catch(error => console.error('Error fetching repositories:', error));
    return latestRepos;
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
        if (repoData == undefined || repoData == null || repoData.length === 0)
            return (<div className="centered">No repositories found</div>);
       
      return (
           <div id="repoDataTable" className="centered">
                <table>
                  <tbody>
                  {
                      repoData.map(function (repo:any){
                          return(
                            <tr key={repo}>
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
