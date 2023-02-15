<script setup lang="ts">
  import type {Todo} from '@/dtos/todo';

  const props = defineProps<{
    todo: Todo,
  }>();

</script>

<template>
  <div class="flex mb-4 items-center">
    <p :class="['w-full', {'line-through': props.todo.completed}]">{{props.todo.description}}</p>


    <button v-if="props.todo.completed"
            class="ml-4 mr-2 btn btn-outline primary"
            name="not_done">Undo</button>
    <button class="ml-4 mr-2 btn btn-outline btn-success"
            name="done">Done</button>

    <button class="ml-2 btn btn-outline btn-error"
            name="remove">Remove</button>


  </div>

</template>



