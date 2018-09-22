import React, { Component } from 'react'
import './App.css'
import Task from './Task'
import AppState from '../store'
import PropTypes from 'prop-types'
import { computed, observable } from 'mobx'
import { observer } from 'mobx-react'

@observer
class App extends Component {
  @observable newTaskName = 'Default'
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
        <div className="AddTask">
          <input id="newTaskName" type="text" value={this.newTaskName} onChange={_ => this.newTaskName = document.getElementById('newTaskName').value}></input>
          <button onClick={_ => this.props.store.addTask(this.newTaskName)}>
            {'Add Task ' + this.newTaskName}
          </button>
        </div>
      </div>
    );
  }
}

App.propTypes = {
  store: PropTypes.instanceOf(AppState)
}

export default App
