import React, { Component } from 'react'
import './Task.css'
import PropTypes from 'prop-types'
import TaskModel from '../models/TaskModel'

class Task extends Component {
  render() {
    return (
      <li className="Task">
        <span className="title-text">{this.props.task.title}</span>
      </li>
    )
  }
}

Task.propTypes = {
  task: PropTypes.instanceOf(TaskModel),
}

export default Task