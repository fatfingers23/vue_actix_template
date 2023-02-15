import { ref, computed } from 'vue'
import { defineStore } from 'pinia'

export const useTodoStore = defineStore('todo', () => {

  //array of objects in typescript

  const todos = ref<Array<Object>>([])

  function addTodo(todo: Object) {
    todos.value.push(todo);
  }

  return { todos, addTodo }
})
