import { action, computed, observable } from 'mobx'
import Task from './Task'

export default class AppState {
  @observable tasks = []

  @computed get completedTasksCount() {
    return this.tasks.filter(
      task => task.completed === true
    ).length;
  }

  @action addTask(title) {
    // Stand-in - this has a backend component to it
    this.tasks.push(new Task(title))
  }
}