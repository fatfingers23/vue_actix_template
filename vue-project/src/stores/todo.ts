import {ref} from 'vue';
import {defineStore} from 'pinia';
import type {Todo} from '@/dtos/todo';

const baseApi = `${import.meta.env.PROD ? '' : import.meta.env.VITE_API_URL ?? ''}/api`;


export const useTodoStore = defineStore('todo', () => {

        const todos = ref<Array<Todo>>([]);

        function simpleFetchWrapper(url: string, verb: string, body?: string): Promise<Response> {
            const requestOptions: RequestInit =
                {
                    method: verb,
                    headers: {
                        'Content-type': 'application/json; charset=UTF-8',
                    }
                };
            if(body){
                requestOptions.body = body;
            }
            return fetch(`${baseApi}${url}`, requestOptions);
        }

        function addTodo(todo: Todo) {
            simpleFetchWrapper('/todo/create','POST', JSON.stringify(todo))
            .then(async response => {
                if(response.ok) {
                    const todo: Todo = await response.json();
                    todos.value.push(todo);
                }
            }).catch(error => {
                alert('Mayday. Something went wrong. Check the console.');
                console.log(error);
            });
        }

        function removeTodo(todo: Todo) {
            simpleFetchWrapper(`/todo/delete/${todo.id}`,'DELETE')
                .then(async response => {
                    if(response.ok){
                        const index = todos.value.indexOf(todo);
                        if (index !== -1) {
                            todos.value.splice(index, 1);
                        }
                    }
                }).catch(error => {
                alert('Mayday. Something went wrong. Check the console.');
                console.log(error);
            });
        }

        function completeTodo(id: number) {
            simpleFetchWrapper(`/todo/complete/${id}`,'PATCH')
                .then(async response => {
                    if(response.ok){
                        const todo = todos.value.find(todo => todo.id === id);
                        if (todo) {
                            todo.completed = true;
                        }
                    }
                }).catch(error => {
                alert('Mayday. Something went wrong. Check the console.');
                console.log(error);
            });
        }

        function loadTodos(){
            simpleFetchWrapper('/todo/list', 'GET').then(async response => {
                if(response.ok) {
                    todos.value = await response.json();
                }
            }).catch(error => {
                alert('Mayday. Something went wrong. Check the console.');
                console.log(error);
            });
        }

        return {todos, addTodo, removeTodo, completeTodo, loadTodos};
    });
