import React, { Component } from 'react'
import './AddTask.css'
import PropTypes from 'prop-types'

class AddTask extends Component {
  constructor(props) {
    super(props)
    this.state = { newTaskName: 'Default' }
  }
  render() {
    return (
      <div className="AddTask">
        <input
          id="newTaskName"
          type="text"
          value={this.state.newTaskName}
          onChange={_ => this.setState({ newTaskName: document.getElementById('newTaskName').value })}>
        </input>
        <button onClick={_ => this.props.addTask(this.state.newTaskName)}>
          {'Add Task ' + this.state.newTaskName}
        </button>
      </div >
    )
  }
}

AddTask.propTypes = {
  addTask: PropTypes.func,
}

export default AddTask