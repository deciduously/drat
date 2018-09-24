import React, { Component } from 'react'
import styles from './Task.css'
import PropTypes from 'prop-types'
import TaskModel from '../store/TaskModel'
import { observer } from 'mobx-react'

const Task = observer(({ task, deleteTask }) =>
  <div className={styles.Task}>
    <span className={styles.titletext}>{task.title}</span><br />
    <button onClick={_ => task.toggleCompleted()}>
      {(task.completed ? 'Un-did it' : 'Did it')}
    </button>
    <button onClick={_ => deleteTask(task.id)}>Delete</button>
  </div>
)

Task.propTypes = {
  task: PropTypes.instanceOf(TaskModel),
  deleteTask: PropTypes.func,
}

export default Task