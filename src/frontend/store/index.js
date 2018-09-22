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
        this.tasks.push(new TaskModel(t.id, r.title, false))
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