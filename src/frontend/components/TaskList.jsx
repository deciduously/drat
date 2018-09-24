import React from 'react'
import styles from './TaskList.css'
import Task from './Task'
import PropTypes from 'prop-types'
import TaskModel from '../store/TaskModel'
import { observer } from 'mobx-react'

const TaskList = observer(({ tasks, header, deleteTask }) =>
  <div className={styles.TaskList}>
    <span className={styles.tasklistheader}>{header}</span>
    <div className={styles.taskcontainer}>
      {tasks.map(task => (
        <Task key={task.id} task={task} deleteTask={event => deleteTask(event)} />
      ))}
    </div>
  </div>
)

TaskList.propTypes = {
  task: PropTypes.arrayOf(PropTypes.instanceOf(TaskModel)),
  header: PropTypes.string,
  deleteTask: PropTypes.func,
}

export default TaskList