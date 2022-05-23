import './App.css';
import './gitProjects'
import {RepoData} from "./gitProjects";
import moment from "moment";
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import { solid, regular, brands } from '@fortawesome/fontawesome-svg-core/import.macro' // <-- import styles to be used

function Header(){
    return (
        <div>
            <h1 id="nameHeader">Nico Judge</h1>
            <h4>Baltimore, MD</h4>
            <table className="center">
                <tr>
                    <td>
                        <a href="https://www.github.com/sirjudge" className="gitIcon"><FontAwesomeIcon icon={brands("github")}/></a>
                    </td>
                    <td>
                        <a href="mailto:nico.a.judge@gmail.com" className="emailIcon"><FontAwesomeIcon icon={solid("envelope")}/></a>
                    </td>
                    <td>
                        <a href="https://www.linkedin.com/in/nicojudge/"><FontAwesomeIcon icon={brands("linkedin")}/></a>
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
            <tr>
                <td className="tableHeader">
                    Repository
                </td>
                <td className="tableHeader">
                    Last Update
                </td>
            </tr>
            {RepoData.map(repo => {
                return (
                    <tr key={repo.id}>
                        <td>
                            <a href={repo.url}>{repo.name}</a>
                        </td>
                        <td className="orangeText">
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

function Bio(){
    return (
        <span className="bio">Enthusiastic full stack web developer with a passion for automation. Check out my projects below!</span>
    );

}

function App() {
  return (
    <div className="App">
        <header className="App-header">
          <Header></Header>
        </header>
        <Bio></Bio>
        <table className="RepoTable">
                <ReturnRepoList></ReturnRepoList>
        </table>

     </div>
  );
}

export default App;
