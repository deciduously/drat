import React, { Component } from 'react'
import './App.css'
import Task from './Task'
import AppState from '../models'

class App extends Component {
  render() {
    return (
      <div className="App">
        <header className="App-header">
          <h1 className="App-title">DRAT</h1>
        </header>
        <p className="App-intro">
          <ul>
            {this.props.store.tasks.map(task => (
              <Task title={task.title} />
            ))}
          </ul>
        </p>
      </div>
    );
  }
}

App.propTypes = {
  store: PropTypes.instanceOf(AppState)
}

export default App
