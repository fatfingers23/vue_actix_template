import { ref } from 'vue';
import { defineStore } from 'pinia';
import type {Todo} from '@/dtos/todo';

export const useTodoStore = defineStore('todo', () => {

  //array of objects in typescript

  const todos = ref<Array<Todo>>([]);

  function addTodo(todo: Todo) {

    todos.value.push(todo);
  }

  return { todos, addTodo };
});
