import './App.css';
import {RepoData} from "./gitProjects";
import moment from "moment";
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import { solid, regular, brands } from '@fortawesome/fontawesome-svg-core/import.macro' // <-- import styles to be used
import mapleNicoPhoto from './images/mapleNicoPhoto.jpg'
import { Grid, Row, Col } from 'react-flexbox-grid';

function Header(){
    return (
            <Grid fluid>
                <Row>
                    <Col className="center">
                        <h1 className="header center">Nico Judge</h1>
                    </Col>
                </Row>
                <Row>
                    <Col className="center">
                        <h4 className="location">Full Stack Software Engineer</h4>
                    </Col>
                </Row>
                <Row>
                    <Col className="center">
                        <Row>
                            <Col>
                                <a href="https://www.github.com/sirjudge" className="gitIcon"><FontAwesomeIcon icon={brands("github")}/></a>
                            </Col>
                            <Col>
                                <a href="mailto:nico.a.judge@gmail.com" className="emailIcon"><FontAwesomeIcon icon={solid("envelope")}/></a>
                            </Col>
                            <Col>
                                <a href="https://www.linkedin.com/in/nicojudge/"  className="linkedInIcon"><FontAwesomeIcon icon={brands("linkedin")}/></a>
                            </Col>
                        </Row>
                    </Col>

                </Row>
            </Grid>

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
    let url = "https://api.github.com/users/sirjudge/repos", repoData = [];
    await GetData(url)
        .then(data => {
            for(var i in data)
                repoData.push([i, data [i]]);
        });

    return (
        <tbody>
            {repoData.map(repo => {
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
        <Grid fluid>
            <Row>
                <Col xs={6}>
                    <Row>
                        <span className="bio">Enthusiastic full stack web developer with a passion for automation. Check out my projects below!</span>
                    </Row>
                    <Row>
                        <ReturnRepoList></ReturnRepoList>
                    </Row>
                </Col>
                <Col>
                    <img className="scaledImage" src={mapleNicoPhoto} alt="Me and My dog Maple"/>
                </Col>
            </Row>
        </Grid>
    );
}

function App() {
  return (
    <div className="App">
        {/*<header className="App-header">*/}
        {/*</header>*/}
        <Header></Header>
        <Bio></Bio>
     </div>
  );
}

export default App;
