import './App.css';
import './gitProjects'
import {RepoData} from "./gitProjects";
import moment from "moment";

function Header(){
    return (
        <div>
            <h1 id="nameHeader">Nico Judge</h1>
            <h4>Baltimore, MD</h4>
            <table>
                <tr>
                    <td>
                        <a href="www.github.com/sirjudge">www.github.com/sirjudge</a>
                    </td>
                    <td>
                        nico.a.judge@gmail.com
                    </td>
                </tr>
            </table>
        </div>
    )
}

async function GetData(url = '', data = {}) {
    const response = await fetch(url, {
        method: 'GET'
    });
    return response.json();// parses JSON response into native JavaScript objects
}

function ReturnRepoList(){
    moment.locale('en');
    return (
        <tbody>
            {RepoData.map(repo => {
                return (
                    <tr key={repo.id}>
                        <td>
                            <a href={repo.url}>{repo.name}</a>
                        </td>
                        <td>
                            {moment(repo.pushed_at).format('MMM D, YYYY')}
                        </td>
                    </tr>
                )
            })}
        </tbody>
    );
}

const GenerateRepoList = async() => {
    var url="https://api.github.com/users/sirjudge/repos";
    var repoData;
    await GetData(url)
        .then(data => {
            var repoData = [];
            for(var i in data)
                repoData.push([i, data [i]]);
        });

    return (
        <tbody>
            {RepoData.map(repo => {
                return (
                    <tr key={repo.id}>
                        <td>
                            <a href={repo.html_url}>{repo.name}</a>
                        </td>
                        <td>
                            {moment(repo.pushed_at).format('MMM D, YYYY')}
                        </td>
                    </tr>
                )
            })}
        </tbody>
    );
}

function App() {
  return (
    <div className="App">
        <header className="App-header">
          <Header></Header>
        </header>
        <table>
                <ReturnRepoList></ReturnRepoList>
        </table>

     </div>
  );
}

export default App;
