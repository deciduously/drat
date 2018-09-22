import React, { Component } from 'react'
import './App.css'
import Task from './Task'
import AppState from '../models'
import PropTypes from 'prop-types'

class App extends Component {
  render() {
    return (
      <div className="App">
        <header className="App-header">
          <h1 className="App-title">DRAT</h1>
        </header>
        <ul>
          {this.props.store.tasks.map(task => (
            <Task key={task.id} task={task} />
          ))}
        </ul>
      </div>
    );
  }
}

App.propTypes = {
  store: PropTypes.instanceOf(AppState)
}

export default App
