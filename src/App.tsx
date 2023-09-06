import React from 'react';
import './App.css';
import './Styles/GlobalStyle.css'

import Header from "./components/Header";
import RepoList from "./components/RepoList";
import AboutMe from "./components/AboutMe";

function App() {
  return (
    <div className="App">
        <script src="https://kit.fontawesome.com/dd6b78a06a.js" crossOrigin="anonymous"></script>
        <Header></Header>
        <AboutMe/>
        <RepoList></RepoList>
    </div>
  );
}

export default App;
