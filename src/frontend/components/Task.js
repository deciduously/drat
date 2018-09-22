import React, { Component } from 'react'
import './Task.css'
import PropTypes from 'prop-types'

class Task extends Component {
  render() {
    return (
      <li className="Task">
        <span className="title-text">{this.props.title}</span>
      </li>
    )
  }
}

Task.propTypes = {
  title: PropTypes.string,
}

export default Task