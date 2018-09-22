import { action, observable } from 'mobx'

var currentID = 0

export default class TaskModel {
    @observable title = ''
    @observable completed = false
    constructor(title) {
        this.title = title
        this.id = currentID
        currentID += 1
    }
    @action toggleCompleted() {
        this.completed = !this.completed
    }
}