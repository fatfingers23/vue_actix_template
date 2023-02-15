<script setup lang="ts">
import {useTodoStore} from "@/stores/todo";
import {computed, ref} from "vue";
import TodoItemComponent from "@/components/todoItemComponent.vue";

const todoStore = useTodoStore();
const todo = ref<Object>({});

const addANewTodo = () => {
  todoStore.addTodo(todo.value)
  todo.value = {}
};

</script>

<template>
  <div class="container "/>

  <div class="h-100 w-full flex items-center justify-center bg-teal-lightest font-sans">
    <div class="rounded shadow p-6 m-4 w-full  lg:max-w-lg">
      <div class="mb-4">
        <div class="flex mt-4 w-full">
          <div class="form-control w-full">
            <div class="input-group">
              <input v-model="todo.description" type="text" placeholder="Add Todo" class="input input-bordered w-full"
                     name="new_todo">
              <button v-on:click="addANewTodo" class="btn btn-square btn-success">
                Add
              </button>
            </div>
          </div>
        </div>
      </div>
      <div>

        <div v-if="todoStore.todos.length === 0" class="flex mb-4 items-center text-center">
          <p class="w-full ">No todos yet! Add one to get started.</p>
        </div>
        <todo-item-component v v-for="(todo, key) in todoStore.todos" :key="key" :todo="todo"/>
      </div>
    </div>
  </div>

</template>


