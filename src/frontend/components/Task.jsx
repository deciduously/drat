import React, { Component } from 'react'
import './Task.css'
import PropTypes from 'prop-types'
import TaskModel from '../store/TaskModel'
import { observer } from 'mobx-react'

@observer
class Task extends Component {
  render() {
    return (
      <li className="Task">
        <span className="title-text">{this.props.task.title}</span>
        <button onClick={_ => this.props.task.toggleCompleted()}>
          {(this.props.task.completed ? 'Un-did it' : 'Did it')}
        </button>
      </li>
    )
  }
}

Task.propTypes = {
  task: PropTypes.instanceOf(TaskModel),
}

export default Task