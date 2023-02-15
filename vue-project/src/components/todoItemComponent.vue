<script setup lang="ts">
  const props = defineProps<{
    todo: Object,
  }>();

</script>

<template>
  <div class="flex mb-4 items-center">
<!--    <p class="w-full {{todo['completed'] ? 'line-through' : ''}}">{{todo["description"]}}</p>-->
        <p class="w-full">{{todo.description}}</p>

<!--    <input name="id" value="{{key}}" hidden />-->

<!--    {% if todo["completed"] %}-->
<!--    <button class="ml-4 mr-2 btn btn-outline primary" name="not_done">Not Done</button>-->

<!--    {% else %}-->
<!--    <button class="ml-4 mr-2 btn btn-outline btn-success" name="done">Done</button>-->

<!--    {% endif %}-->

    <button class="ml-2 btn btn-outline btn-error" name="remove">Remove</button>


  </div>

</template>



