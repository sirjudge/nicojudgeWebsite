import logo from './logo.svg';
import './App.css';
import { data } from "./gitProjects"

function Header(){
    return (
        <div>
            <h1 id="nameHeader">Nico Judge</h1>
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

//TODO: This is stack overflow code. This does the await
//https://stackoverflow.com/questions/48969495/in-javascript-how-do-i-should-i-use-async-await-with-xmlhttprequest
function makeRequest(method, url) {
    return new Promise(function (resolve, reject) {
        let xhr = new XMLHttpRequest();
        xhr.open(method, url);
        xhr.onload = function () {
            if (this.status >= 200 && this.status < 300) {
                resolve(xhr.response);
            } else {
                reject({
                    status: this.status,
                    statusText: xhr.statusText
                });
            }
        };
        xhr.onerror = function () {
            reject({
                status: this.status,
                statusText: xhr.statusText
            });
        };
        xhr.send();
    });
}

function GetGitJsonXhrRequest(){
    const url = "https://api.github.com/users/sirjudge/repos?callback=CALLBACK";

    let xhr = new XMLHttpRequest();
    xhr.open('get',url);
    xhr.send();

    var returnJson;
    xhr.onload = function() {
        returnJson = JSON.parse( xhr.response);
    };

    while (returnJson === null){

    }
}

function GetGitJson(){
    var url = "https://api.github.com/users/sirjudge/repos?callback=CALLBACK";
    fetch(url,
        {
            method: "GET"
        })
        .then(function(res){

            console.log(res.json());
            return res.json();

        })
        .then(function(data){ console.log(( JSON.stringify( data ) ) )})
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
