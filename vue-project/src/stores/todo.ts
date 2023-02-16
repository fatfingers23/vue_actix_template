import { ref } from 'vue';
import { defineStore } from 'pinia';
import type {Todo} from '@/dtos/todo';

export const useTodoStore = defineStore('todo', () => {

  //array of objects in typescript

  const todos = ref<Array<Todo>>([]);

  function addTodo(todo: Todo) {
    //TODO make fake id right now. Will get id from api in long run
    todo.id = Math.floor(Math.random() * 10000000);
    todos.value.push(todo);
  }

  function removeTodo(todo: Todo) {
    const index = todos.value.indexOf(todo);
    if (index !== -1) {
      todos.value.splice(index, 1);
    }

  }

  function completeTodo(id: number) {
    const todo = todos.value.find(todo => todo.id === id);
    if(todo){
      todo.completed = true;
    }
  }

  return { todos, addTodo, removeTodo, completeTodo };
});
