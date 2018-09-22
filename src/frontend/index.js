import React from 'react'
import ReactDOM from 'react-dom'
import './index.css'
import App from './components/App'
import AppState from './models'
import DevTools from 'mobx-react-devtools'


const store = new AppState()

store.addTask("MOBX TASK")
store.addTask("TWO OF THEM")

ReactDOM.render(
  <div id="app">
    <App store={store} />
    <DevTools />
  </div>,
  document.getElementById('root'))
