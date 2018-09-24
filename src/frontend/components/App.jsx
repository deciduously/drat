import React, { Component } from 'react'
import './App.css'
import TaskList from './TaskList'
import AppState from '../store'
import PropTypes from 'prop-types'
import { observable } from 'mobx'
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
        <button onClick={_ => this.props.store.refreshTasks()}>Refresh tasks</button>

        <div className="AddTask">
          <input id="newTaskName" type="text" value={this.newTaskName} onChange={_ => this.newTaskName = document.getElementById('newTaskName').value}></input>
          <button onClick={_ => this.props.store.addTask(this.newTaskName)}>
            {'Add Task ' + this.newTaskName}
          </button>
          <TaskList tasks={this.props.store.uncompletedTasks} header='To Do:' deleteTask={id => this.props.store.deleteTask(id)} />
          <TaskList tasks={this.props.store.completedTasks} header='Complete:' deleteTask={id => this.props.store.deleteTask(id)} />
        </div>
      </div>
    );
  }
}

App.propTypes = {
  store: PropTypes.instanceOf(AppState)
}

export default App
