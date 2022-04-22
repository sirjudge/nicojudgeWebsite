import logo from './logo.svg';
import './App.css';
import { data } from "./gitProjects"

function Header(){
    return (
        <div>
            <h1>Nico Judge</h1>
            <h4>Baltimore, MD</h4>
            <table>
                <tr>
                    <td>
                        www.github.com/sirjudge
                    </td>
                    <td>
                        nico.a.judge@gmail.com
                    </td>
                </tr>
            </table>
        </div>
    )
}

function RepoList(repos){
    var gitData = GetGitJson();
    return (
        data.map(function(repo){
              const { id,url,name} = repo;
              return (
                  <tr>
                      <td>{name}</td>
                      <td><a href={url}>{url}</a></td>
                  </tr>
              )
        })
    )
}

function GetGitJson(){
    var xhr = new XMLHttpRequest();
    var url = "https://api.github.com/users/sirjudge/repos?callback=CALLBACK";
    xhr.open("GET", url, true);
    xhr.setRequestHeader("Content-Type", "application/json");
    xhr.onreadystatechange = function () {
        if (xhr.readyState === 4 && xhr.status === 200) {
            var json = JSON.parse(xhr.responseText);
            return json;
        }
    };
    xhr.send();
}

function App() {
  return (
    <div className="App">
        <header className="App-header">
          <Header></Header>
        </header>
        <table>
            <tbody>
                <RepoList></RepoList>
            </tbody>
        </table>
     </div>
  );
}

export default App;
