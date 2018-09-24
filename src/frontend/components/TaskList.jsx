import React, { Component } from 'react'
import './TaskList.css'
import Task from './Task'
import PropTypes from 'prop-types'
import TaskModel from '../store/TaskModel'
import { observer } from 'mobx-react'

const Completed = observer(({ tasks, header, deleteTask }) =>
  <div className="TaskList">
    <span className="tasklist-header">{header}</span>
    <ul>
      {tasks.map(task => (
        <Task key={task.id} task={task} deleteTask={event => deleteTask(event)} />
      ))}
    </ul>
  </div>
)

Completed.propTypes = {
  task: PropTypes.arrayOf(PropTypes.instanceOf(TaskModel)),
  header: PropTypes.string,
  deleteTask: PropTypes.func,
}

export default Completed