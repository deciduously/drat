import React from 'react'
import ReactDOM from 'react-dom'
import './index.css'
import App from './components/App'
import registerServiceWorker from './registerServiceWorker'
import AppState from './models'

const store = new AppState()

store.addTask("MOBX TASK")
store.addTask("TWO OF THEM")

ReactDOM.render(<App store={store} />, document.getElementById('root'))
registerServiceWorker()
