import { action, computed, observable } from 'mobx'

class TaskInstance {
  @observable name = ''
}

class TaskStore {
  @observable tasks = []

  constructor() {
    mobx.autorun(() => console.log(this.report))
  }

  @computed get completedTasksCount() {
    return this.tasks.filter(
      task => task.completed === true
    ).length;
  }

  @computed get report() {
    if (this.tasks.length === 0)
      return "<none>";
    return `Next task: "${this.tasks[0].title}". ` +
      `Progress: ${this.completedTasksCount}/${this.tasks.length}`
  }

  @action addTask(title) {
    this.tasks.push({
      title: title,
      completed: false,
    })
  }
}

const taskStore = new TaskStore()