import './App.css';

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

// async function ExtractGitRepoList(){
async function RepoList(){
    var url="https://api.github.com/users/sirjudge/repos";
    var repoData;
    await GetData(url)
        .then(data => {
            repoData = data;
        });

    return (
        <table>
            <tbody>
            {repoData.map(repo => {
                    return (
                    <tr key={repo.id}>
                        <td>
                            {repo.name}
                        </td>
                        <td>
                            <a href={repo.url}></a>
                        </td>
                    </tr>
                    )
                })}
            </tbody>
        </table>
    );
}

function App() {
  return (
    <div className="App">
        <header className="App-header">
          <Header></Header>
        </header>
        <RepoList></RepoList>
     </div>
  );
}

export default App;
