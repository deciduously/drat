import React, { Component } from 'react'
import './Task.css'
import PropTypes from 'prop-types'

class Task extends Component {
  render() {
    return (
      <div className="Task">
        <span className="title-text">{this.props.title}</span>
      </div>
    )
  }
}

Task.propTypes = {
  title: PropTypes.string,
}

export default Task