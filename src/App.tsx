import React from 'react';
import logo from './logo.svg';
import './App.css';
import Header from "./components/Header";
import RepoList from "./components/RepoList";

function App() {
  return (
    <div className="App">
        <script src="https://kit.fontawesome.com/dd6b78a06a.js" crossOrigin="anonymous"></script>
        <Header></Header>
        <RepoList></RepoList>
    </div>
  );
}

export default App;
