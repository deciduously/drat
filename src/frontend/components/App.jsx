import React, { Component } from 'react'
import './App.css'
import AddTask from './AddTask'
import TaskList from './TaskList'
import AppState from '../store'
import PropTypes from 'prop-types'
import { observable } from 'mobx'
import { observer } from 'mobx-react'

@observer
class App extends Component {

  render() {
    return (
      <div className="App">
        <header className="App-header">
          <h1 className="App-title">DRAT</h1>
        </header>
        <button onClick={_ => this.props.store.refreshTasks()}>Refresh tasks</button>
        <AddTask addTask={title => this.props.store.addTask(title)} />
        <TaskList
          tasks={this.props.store.uncompletedTasks}
          header='To Do:'
          deleteTask={id => this.props.store.deleteTask(id)} />
        <TaskList
          tasks={this.props.store.completedTasks}
          header='Complete:'
          deleteTask={id => this.props.store.deleteTask(id)} />
      </div>
    );
  }
}

App.propTypes = {
  store: PropTypes.instanceOf(AppState)
}

export default App
