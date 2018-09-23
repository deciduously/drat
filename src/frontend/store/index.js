import { action, computed, observable } from 'mobx'
import TaskModel from './TaskModel'

export default class AppState {
  @observable tasks = []

  constructor() {
    this.refreshTasks()
  }

  @computed get completedTasksCount() {
    return this.tasks.filter(
      task => task.completed === true
    ).length;
  }

  @action addTask(title) {
    fetch('http://localhost:8080/task/new/' + title)
      .then(res => {
        if (res.ok) {
          return res.json()
        }

        throw new Error('Network response was not ok')
      }).then(r => {
        this.tasks.push(new TaskModel(r.id, r.title, r.completed))
      }).catch(e => {
        console.log('Failed fetch operation: ', e.message)
      })
  }

  @action deleteTask(id) {
    fetch('http://localhost:8080/task/delete/' + id)
      .then(res => {
        if (res.ok) {
          return 'ok!'
        }

        throw new Error('Network response was not ok')
      }).then(_ => {
        var idx = -1
        const len = this.tasks.length
        // find the task in the store
        for (var i = 0; i < len; ++i) {
          if (this.tasks[i].id === id) {
            idx = i
          }
        }
        // remove it
        if (idx !== -1) {
          this.tasks.splice(idx, 1)
        }
      }).catch(e => {
        console.log('Failed fetch operation: ', e.message)
      })
  }

  @action refreshTasks() {
    // clear it all out and then reread
    this.tasks = []
    fetch('http://localhost:8080/task/all')
      .then(res => {
        if (res.ok) {
          return res.json()
        }

        throw new Error('Network response was not ok')
      }).then(r => {
        r.tasks.map(t => {
          this.tasks.push(new TaskModel(t.id, t.title, t.completed))
        })
      }).catch(e => {
        console.log('Failed fetch operation: ', e.message)
      })
  }
}