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
        this.completed = !this.completed
    }
}