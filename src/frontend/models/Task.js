import { action, observable } from 'mobx'

export default class Task {
    @observable title = ''
    @observable completed = false
    constructor(title) {
        this.title = title
    }
    @action toggleCompleted() {
        this.completed = !this.completed
    }
}