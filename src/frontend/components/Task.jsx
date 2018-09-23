import React, { Component } from 'react'
import './Task.css'
import PropTypes from 'prop-types'
import TaskModel from '../store/TaskModel'
import { observer } from 'mobx-react'

const Task = observer(({ task, deleteTask }) =>
  <li className="Task">
    <span className="title-text">{task.title}</span>
    <button onClick={_ => task.toggleCompleted()}>
      {(task.completed ? 'Un-did it' : 'Did it')}
    </button>
    <button onClick={_ => deleteTask(task.id)}>Delete</button>
  </li>
)

Task.propTypes = {
  task: PropTypes.instanceOf(TaskModel),
  deleteTask: PropTypes.func,
}

export default Task