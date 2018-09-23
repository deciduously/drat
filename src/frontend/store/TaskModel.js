import { action, observable } from 'mobx'

export default class TaskModel {
  @observable title = ''
  @observable completed = false
  constructor(id, title, completed) {
    this.title = title
    this.id = id
    this.completed = completed
  }
  @action toggleCompleted() {
    fetch('http://localhost:8080/task/toggle/' + this.id)
      .then(res => {
        if (res.ok) {
          return res.json()
        }

        throw new Error('Network response was not ok')
      }).then(r => {
        this.completed = r.completed
      }).catch(e => {
        console.log('Failed fetch operation: ', e.message)
      })
  }
}