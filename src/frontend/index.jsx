import React from 'react'
import ReactDOM from 'react-dom'
import './index.css'
import App from './components/App'
import AppState from './store'
import DevTools from 'mobx-react-devtools'

// maybe move this into componentDidMount? 
const store = new AppState()

ReactDOM.render(
  <div id='app'>
    <App store={store} />
    <DevTools />
  </div>,
  document.getElementById('root'))
