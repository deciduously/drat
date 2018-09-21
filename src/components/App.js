import React, { Component } from 'react'
import './App.css'
import Task from './Task'

class App extends Component {
  render() {
    return (
      <div className="App">
        <header className="App-header">
          <h1 className="App-title">DRAT</h1>
        </header>
        <p className="App-intro">
          <Task title="TASK 1 TITLE"/>
        </p>
      </div>
    );
  }
}

export default App
